use async_graphql::Result;
use reqwest::{Method, StatusCode};
use sha2::{Digest, Sha256};

use super::request::make_request;
use super::ty::user::{UserRequest, UsersRequest, UsersResponse};

pub(crate) struct UserService<'a> {
    token: &'a str,
}

impl<'a> UserService<'a> {
    pub async fn new(token: &'a str) -> Self {
        Self { token }
    }

    /// Add users by emails
    pub async fn add_users(
        &self,
        segment_id: String,
        emails: Vec<String>,
    ) -> Result<UsersResponse> {
        let path = &format!("/segments/{segment_id}/users");
        let mut data: Vec<Vec<String>> = Vec::new();
        for email in emails.iter() {
            let mut hasher = Sha256::new();
            hasher.update(email);
            let hashed_email = hasher.finalize();
            let hex_data = hex::encode(hashed_email);
            data.push(vec![hex_data]);
        }
        let body = serde_json::to_string(&UsersRequest {
            users: vec![UserRequest {
                schema: vec!["EMAIL_SHA256".to_string()],
                data,
            }],
        })?;
        let res = make_request(self.token, Method::POST, path, Some(body)).await?;
        if !matches!(res.status(), StatusCode::OK) {
            return Err(async_graphql::Error::new(res.text().await?));
        }
        Ok(res.json().await?)
    }
}
