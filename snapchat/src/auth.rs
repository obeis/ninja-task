use std::collections::HashMap;

use async_graphql::Result;
use reqwest::{Method, StatusCode};

use crate::request::make_form_request;

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
        let mut form = HashMap::new();
        form.insert("grant_type", "authorization_code");
        form.insert("client_id", self.client_id);
        form.insert("client_secret", self.client_secret);
        form.insert("code", code);
        form.insert("redirect_uri", self.redirect_uri);
        let res = make_form_request(Method::POST, "/login/oauth2/access_token", Some(form)).await?;
        if !matches!(res.status(), StatusCode::OK) {
            return Err(async_graphql::Error::new(res.text().await?));
        }
        Ok(res.json().await?)
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
        Ok(res.json().await?)
    }
}
