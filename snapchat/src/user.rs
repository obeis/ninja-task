use async_graphql::Result;
use reqwest::{Method, StatusCode};
use sha2::{Digest, Sha256};
use tracing::info;

use super::request::make_request;
use super::ty::segment::SegmentsResponse;
use super::ty::user::{UserRequest, UsersRequest, UsersResponse};

pub struct UserService<'a> {
    token: &'a str,
}

pub enum UserIdentifier {
    Unknown,
    Email,
    MobileAdId,
    PhoneNumber,
}

impl UserIdentifier {
    pub fn to_schema(&self) -> String {
        match self {
            UserIdentifier::Email => "EMAIL_SHA256".to_string(),
            UserIdentifier::MobileAdId => "MOBILE_AD_ID_SHA256".to_string(),
            UserIdentifier::PhoneNumber => "PHONE_SHA256".to_string(),
            UserIdentifier::Unknown => "".to_string(),
        }
    }

    pub fn from_string(s: String) -> Self {
        match s.as_str() {
            "email" => UserIdentifier::Email,
            "mobile" => UserIdentifier::MobileAdId,
            "phone" => UserIdentifier::PhoneNumber,
            _ => UserIdentifier::Unknown,
        }
    }
}

impl<'a> UserService<'a> {
    pub async fn new(token: &'a str) -> Self {
        Self { token }
    }

    /// Add users by identifier (email/mobile ad id/phone number)
    pub async fn add_users(
        &self,
        segment_id: String,
        identifiers: Vec<String>,
        schema_ty: UserIdentifier,
    ) -> Result<UsersResponse> {
        let path = &format!("/segments/{segment_id}/users");
        let data = hash_identifiers(identifiers);
        let body = serde_json::to_string(&UsersRequest {
            users: vec![UserRequest {
                schema: vec![schema_ty.to_schema()],
                data,
            }],
        })?;
        info!("reqeust body: `{}`", body);
        let res = make_request(self.token, Method::POST, path, Some(body)).await?;
        if !matches!(res.status(), StatusCode::OK) {
            return Err(async_graphql::Error::new(res.text().await?));
        }
        let text = res.text().await?;
        info!("text response: `{}`", text);
        Ok(serde_json::from_str(&text)?)
    }

    /// Delete users by identifier (email/mobile ad id/phone number)
    pub async fn delete_users(
        &self,
        segment_id: String,
        identifiers: Vec<String>,
        schema_ty: UserIdentifier,
    ) -> Result<UsersResponse> {
        let path = &format!("/segments/{segment_id}/users");
        let data = hash_identifiers(identifiers);
        let body = serde_json::to_string(&UsersRequest {
            users: vec![UserRequest {
                schema: vec![schema_ty.to_schema()],
                data,
            }],
        })?;
        info!("reqeust body: `{}`", body);
        let res = make_request(self.token, Method::DELETE, path, Some(body)).await?;
        if !matches!(res.status(), StatusCode::OK) {
            return Err(async_graphql::Error::new(res.text().await?));
        }
        let text = res.text().await?;
        info!("text response: `{}`", text);
        Ok(serde_json::from_str(&text)?)
    }

    /// Delete all users from segment
    pub async fn delete_all_users(&self, segment_id: String) -> Result<SegmentsResponse> {
        let path = &format!("/segments/{segment_id}/all_users");
        let res = make_request(self.token, Method::DELETE, path, None).await?;
        if !matches!(res.status(), StatusCode::OK) {
            return Err(async_graphql::Error::new(res.text().await?));
        }
        let text = res.text().await?;
        info!("text response: `{}`", text);
        Ok(serde_json::from_str(&text)?)
    }
}

fn hash_identifiers(identifiers: Vec<String>) -> Vec<Vec<String>> {
    let mut data: Vec<Vec<String>> = Vec::new();
    for ident in identifiers.iter() {
        let mut hasher = Sha256::new();
        hasher.update(ident);
        let hashed_idents = hasher.finalize();
        let hex_data = hex::encode(hashed_idents);
        data.push(vec![hex_data]);
    }
    data
}
