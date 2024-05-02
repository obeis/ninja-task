use std::collections::HashMap;

use async_graphql::Result;
use reqwest::{Method, StatusCode};
use tracing::{event, Level};

use crate::request::make_form_request;

use super::ty::auth::Token;

pub struct AuthService<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    refresh_token: &'a str,
}

impl<'a> AuthService<'a> {
    pub async fn new(client_id: &'a str, client_secret: &'a str, refresh_token: &'a str) -> Self {
        Self {
            client_id,
            client_secret,
            refresh_token,
        }
    }

    pub async fn refresh_token(&self) -> Result<Token> {
        let mut form = HashMap::new();
        form.insert("grant_type", "refresh_token");
        form.insert("client_id", self.client_id);
        form.insert("client_secret", self.client_secret);
        form.insert("refresh_token", self.refresh_token);
        let res = make_form_request(Method::POST, "/login/oauth2/access_token", Some(form)).await?;
        if !matches!(res.status(), StatusCode::OK) {
            return Err(async_graphql::Error::new(res.text().await?));
        }
        event!(Level::INFO, "refersh token");
        Ok(res.json().await?)
    }
}
