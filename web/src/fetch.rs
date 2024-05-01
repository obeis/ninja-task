use anyhow::Result;
use async_recursion::async_recursion;
use reqwest::Response;
use serde_json::Value;

use snapchat::ty::auth::Token;
use snapchat::ty::segment::{SegmentResponse, SegmentsRequest};
use snapchat::ty::user::UserResponse;

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

const CREATE_SEGMENTS_QUERY: &str = r#"{"query":"mutation {\ncreateSegments(segments: VAR_SEGMENTS) {\nsegments {\nsub_request_status\nsegment {\nid\nname\ndescription\nstatus\nsource_type\nad_account_id\norganization_id\ntargetable_status\nupload_status\nretention_in_days\napproximate_number_users\nvisible_to\ncreated_at\nupdated_at\n}\n}\n}\n}"}"#;

#[async_recursion(?Send)]
pub async fn create_segments(segments: SegmentsRequest) -> Result<Vec<SegmentResponse>> {
    let res = fetch(CREATE_SEGMENTS_QUERY.replace(
        "VAR_SEGMENTS",
        &serde_json::to_string(&segments)?.replace('"', r#"\""#),
    ))
    .await?;
    let text = res.text().await?;
    let root: Value = serde_json::from_str(&text)?;
    if is_auth_err(root.clone()).await? {
        return create_segments(segments).await;
    }
    let payload = root
        .get("data")
        .and_then(|value| value.get("createSegment"))
        .and_then(|value| value.get("segments"));
    Ok(serde_json::from_str(&payload.unwrap().to_string())?)
}

const ADD_USERS: &str = r#"{"query":"mutation {\naddUsers(segmentId: \"SEGMENT_ID\", emails: EMAILS) {\nrequest_status\nrequest_id\nusers {\nsub_request_status\nuser {\nnumber_uploaded_users\n}\n}\n}\n}"}"#;

#[async_recursion(?Send)]
pub async fn add_users(segment_id: String, emails: Vec<String>) -> Result<Vec<UserResponse>> {
    let res = fetch(ADD_USERS.replace("SEGMENT_ID", &segment_id).replace(
        "EMAILS",
        &serde_json::to_string(&emails)?.replace('"', r#"\""#),
    ))
    .await?;
    let text = res.text().await?;
    let root: Value = serde_json::from_str(&text)?;
    if is_auth_err(root.clone()).await? {
        return add_users(segment_id, emails).await;
    }
    let payload = root
        .get("data")
        .and_then(|value| value.get("addUsers"))
        .and_then(|value| value.get("users"));
    Ok(serde_json::from_str(&payload.unwrap().to_string())?)
}

const DELETE_USERS: &str = r#"{"query":"mutation {\ndeleteUsers(segmentId: \"SEGMENT_ID\", emails: EMAILS) {\nrequest_status\nrequest_id\nusers {\nsub_request_status\nuser {\nnumber_uploaded_users\n}\n}\n}\n}"}"#;

#[async_recursion(?Send)]
pub async fn delete_users(segment_id: String, emails: Vec<String>) -> Result<Vec<UserResponse>> {
    let res = fetch(DELETE_USERS.replace("SEGMENT_ID", &segment_id).replace(
        "EMAILS",
        &serde_json::to_string(&emails)?.replace('"', r#"\""#),
    ))
    .await?;
    let text = res.text().await?;
    let root: Value = serde_json::from_str(&text)?;
    if is_auth_err(root.clone()).await? {
        return delete_users(segment_id, emails).await;
    }
    let payload = root
        .get("data")
        .and_then(|value| value.get("deleteUsers"))
        .and_then(|value| value.get("users"));
    Ok(serde_json::from_str(&payload.unwrap().to_string())?)
}

const DELETE_ALL_USER_QUERY: &str = r#"{"query":"mutation {\ndeleteAllUsers(segmentId: \"SEGMENT_ID\") {\nsegments {\nsub_request_status\nsegment {\nid\nname\ndescription\nstatus\nsource_type\nad_account_id\norganization_id\ntargetable_status\nupload_status\nretention_in_days\napproximate_number_users\nvisible_to\ncreated_at\nupdated_at\n}\n}\n}\n}"}"#;

#[async_recursion(?Send)]
pub async fn delete_all_users(id: String) -> Result<Vec<SegmentResponse>> {
    let res = fetch(DELETE_ALL_USER_QUERY.replace("SEGMENT_ID", &id)).await?;
    let text = res.text().await?;
    let root: Value = serde_json::from_str(&text)?;
    if is_auth_err(root.clone()).await? {
        return delete_all_users(id).await;
    }
    let payload = root
        .get("data")
        .and_then(|value| value.get("deleteAllUsers"))
        .and_then(|value| value.get("segments"));
    Ok(serde_json::from_str(&payload.unwrap().to_string())?)
}
