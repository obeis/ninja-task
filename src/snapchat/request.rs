use async_graphql::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, Method, Response, Url};

const BASE_URL_V1: &str = "https://adsapi.snapchat.com/v1";

pub(crate) async fn make_request(
    token: &str,
    method: Method,
    path: &str,
    body: String,
) -> Result<Response> {
    let url = Url::parse(&format!("{}{}", BASE_URL_V1, path))?;
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json")?);
    let req = client.request(method, url).headers(headers).body(body);
    let res = req.send().await?;
    Ok(res)
}
