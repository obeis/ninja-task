use std::collections::HashMap;

use async_graphql::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Client, Method, StatusCode, Url};

use super::request::BASE_URL_V1;
use super::ty::auth::Token;

pub struct AuthService<'a> {
    token: &'a str,
    client_id: &'a str,
    client_secret: &'a str,
    redirect_uri: &'a str,
}

impl<'a> AuthService<'a> {
    pub async fn new(
        token: &'a str,
        client_id: &'a str,
        client_secret: &'a str,
        redirect_uri: &'a str,
    ) -> Self {
        Self {
            token,
            client_id,
            client_secret,
            redirect_uri,
        }
    }

    pub async fn generate_token(&self, code: &str) -> Result<Token> {
        let url = Url::parse(&format!("{}{}", BASE_URL_V1, "/login/oauth2/access_token"))?;
        let client = Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.token))?,
        );
        let mut form_data = HashMap::new();
        form_data.insert("grant_type", "authorization_code");
        form_data.insert("client_id", self.client_id);
        form_data.insert("client_secret", self.client_secret);
        form_data.insert("code", code);
        form_data.insert("redirect_uri", self.redirect_uri);
        let res = client
            .request(Method::POST, url)
            .headers(headers)
            .form(&form_data)
            .send()
            .await?;
        if !matches!(res.status(), StatusCode::OK) {
            return Err(async_graphql::Error::new(res.text().await?));
        }
        Ok(res.json().await?)
    }
}
