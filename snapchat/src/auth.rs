use std::collections::HashMap;

use async_graphql::Result;
use reqwest::{Client, Method, StatusCode, Url};

use super::request::BASE_URL_V1;
use super::ty::auth::Token;

pub struct AuthService<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    redirect_uri: &'a str,
    refresh_token: &'a str,
}

impl<'a> AuthService<'a> {
    pub async fn new(
        client_id: &'a str,
        client_secret: &'a str,
        redirect_uri: &'a str,
        refresh_token: &'a str,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri,
            refresh_token,
        }
    }

    pub async fn generate_token(&self, code: &str) -> Result<Token> {
        let url = Url::parse(&format!("{}{}", BASE_URL_V1, "/login/oauth2/access_token"))?;
        let client = Client::new();
        let mut form_data = HashMap::new();
        form_data.insert("grant_type", "authorization_code");
        form_data.insert("client_id", self.client_id);
        form_data.insert("client_secret", self.client_secret);
        form_data.insert("code", code);
        form_data.insert("redirect_uri", self.redirect_uri);
        let res = client
            .request(Method::POST, url)
            .form(&form_data)
            .send()
            .await?;
        if !matches!(res.status(), StatusCode::OK) {
            return Err(async_graphql::Error::new(res.text().await?));
        }
        Ok(res.json().await?)
    }

    pub async fn refresh_token(&self) -> Result<Token> {
        let url = Url::parse(&format!("{}{}", BASE_URL_V1, "/login/oauth2/access_token"))?;
        let client = Client::new();
        let mut form_data = HashMap::new();
        form_data.insert("grant_type", "refresh_token");
        form_data.insert("client_id", self.client_id);
        form_data.insert("client_secret", self.client_secret);
        form_data.insert("refresh_token", self.refresh_token);
        let res = client
            .request(Method::POST, url)
            .form(&form_data)
            .send()
            .await?;
        if !matches!(res.status(), StatusCode::OK) {
            return Err(async_graphql::Error::new(res.text().await?));
        }
        Ok(res.json().await?)
    }
}
