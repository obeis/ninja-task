use std::collections::HashMap;

use async_graphql::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Method, Response, Url};

pub const BASE_URL_V1: &str = "https://adsapi.snapchat.com/v1";
pub const BASE_URL_ACCOUNT: &str = "https://accounts.snapchat.com";

pub async fn make_request(
    token: &str,
    method: Method,
    path: &str,
    body: Option<String>,
) -> Result<Response> {
    let url = Url::parse(&format!("{}{}", BASE_URL_V1, path))?;
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token))?,
    );
    if body.is_some() {
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);
    }
    let req = if let Some(body) = body {
        client.request(method, url).headers(headers).body(body)
    } else {
        client.request(method, url).headers(headers)
    };
    let res = req.send().await?;
    Ok(res)
}

pub async fn make_form_request(
    method: Method,
    path: &str,
    body: Option<HashMap<&str, &str>>,
) -> Result<Response> {
    let url = Url::parse(&format!("{}{}", BASE_URL_ACCOUNT, path))?;
    let client = Client::new();
    let mut headers = HeaderMap::new();
    if body.is_some() {
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str("application/x-www-form-urlencoded")?,
        );
    }
    let req = if let Some(body) = body {
        client.request(method, url).headers(headers).form(&body)
    } else {
        client.request(method, url).headers(headers)
    };
    let res = req.send().await?;
    Ok(res)
}
