use anyhow::Result;
use async_recursion::async_recursion;
use reqwest::Response;
use serde_json::Value;

use snapchat::ty::{auth::Token, segment::SegmentResponse};

const API_URL: &str = "http://localhost:4000/graphql";

pub async fn fetch(query: String) -> Result<Response> {
    let client = reqwest::Client::new();
    Ok(client
        .post(API_URL)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(query)
        .send()
        .await?)
}

const GET_SEGMENTS_QUERY: &str = r#"{"query":"query {\ngetSegments {\nsegments {\nsub_request_status\nsegment {\nid\nname\ndescription\nstatus\nsource_type\nad_account_id\norganization_id\ntargetable_status\nupload_status\nretention_in_days\napproximate_number_users\nvisible_to\ncreated_at\nupdated_at\n}\n}\n}\n}"}"#;

#[async_recursion(?Send)]
pub async fn get_segments() -> Result<Vec<SegmentResponse>> {
    let res = fetch(GET_SEGMENTS_QUERY.to_string()).await?;
    let text = res.text().await?;
    let root: Value = serde_json::from_str(&text)?;
    if is_auth_err(root.clone()).await? {
        return get_segments().await;
    }
    let payload = root
        .get("data")
        .and_then(|value| value.get("getSegments"))
        .and_then(|value| value.get("segments"));
    Ok(serde_json::from_str(&payload.unwrap().to_string())?)
}

async fn is_auth_err(root: Value) -> Result<bool> {
    let err = root.get("errors");
    if err.is_some() {
        for e in err.unwrap().as_array().unwrap().iter() {
            if let Some(value) = e.get("message")
                && let Some("unauthorized") = value.as_str()
            {
                refresh_token().await?;
                return Ok(true);
            }
        }
    }
    Ok(false)
}

const REFRESH_TOKEN_QUERY: &str = r#"{"query":"query {\nrefreshToken {\naccess_token\nrefresh_token\ntoken_type\nexpires_in\nscope\n}\n}"}"#;

pub async fn refresh_token() -> Result<Token> {
    let res = fetch(REFRESH_TOKEN_QUERY.to_string()).await?;
    let text = res.text().await?;
    let root: Value = serde_json::from_str(&text)?;
    let payload = root.get("data").and_then(|value| value.get("refreshToken"));
    Ok(serde_json::from_str(&payload.unwrap().to_string())?)
}

const GET_SEGMENT_QUERY: &str = r#"{"query":"query {\ngetSegment(segmentId: \"SEGMENT_ID\") {\nsegments {\nsub_request_status\nsegment {\nid\nname\ndescription\nstatus\nsource_type\nad_account_id\norganization_id\ntargetable_status\nupload_status\nretention_in_days\napproximate_number_users\nvisible_to\ncreated_at\nupdated_at\n}\n}\n}\n}"}"#;

#[async_recursion(?Send)]
pub async fn get_segment(id: String) -> Result<Vec<SegmentResponse>> {
    let res = fetch(GET_SEGMENT_QUERY.replace("SEGMENT_ID", &id)).await?;
    let text = res.text().await?;
    web_sys::console::log_1(
        &format!("{}\n{}", text, GET_SEGMENT_QUERY.replace("SEGMENT_ID", &id)).into(),
    );
    let root: Value = serde_json::from_str(&text)?;
    if is_auth_err(root.clone()).await? {
        return get_segment(id).await;
    }
    let payload = root
        .get("data")
        .and_then(|value| value.get("getSegment"))
        .and_then(|value| value.get("segments"));
    Ok(serde_json::from_str(&payload.unwrap().to_string())?)
}