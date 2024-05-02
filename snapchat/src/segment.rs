use async_graphql::{Error, Result};
use reqwest::{Method, StatusCode};
use tracing::{info, span, Level};

use super::request::make_request;
use super::ty::segment::{
    SegmentRequest, SegmentsRequest, SegmentsResponse, UpdateSegmentRequest, UpdateSegmentsRequest,
};

#[derive(Debug)]
pub struct SegmentService<'a> {
    token: &'a str,
}

impl<'a> SegmentService<'a> {
    pub async fn new(token: &'a str) -> Self {
        Self { token }
    }

    // Create a segment
    pub async fn create(&self, segments: Vec<SegmentRequest>) -> Result<SegmentsResponse> {
        let span = span!(Level::INFO, "create_segment");
        let _guard = span.enter();

        let ad_account_id = if let Some(seg) = segments.last() {
            &seg.ad_account_id
        } else {
            return Err(Error::new("empty segment list"));
        };
        let path = &format!("/adaccounts/{ad_account_id}/segments");
        let body = serde_json::to_string(&SegmentsRequest { segments })?;
        info!("reqeust body: `{}`", body);
        let res = make_request(self.token, Method::POST, path, Some(body)).await?;
        if !matches!(res.status(), StatusCode::OK) {
            return Err(async_graphql::Error::new(res.text().await?));
        }
        let text = res.text().await?;
        info!("text response: `{}`", text);
        Ok(serde_json::from_str(&text)?)
    }

    // Get all segment
    pub async fn get_all(&self, ad_account_id: &str) -> Result<SegmentsResponse> {
        let span = span!(Level::INFO, "get_all_segment");
        let _guard = span.enter();

        let path = &format!("/adaccounts/{ad_account_id}/segments");
        let res = make_request(self.token, Method::GET, path, None).await?;
        if !matches!(res.status(), StatusCode::OK) {
            return Err(async_graphql::Error::new(res.text().await?));
        }
        let text = res.text().await?;
        info!("text response: `{}`", text);
        Ok(serde_json::from_str(&text)?)
    }

    // Get a segment
    pub async fn get(&self, segment_id: String) -> Result<SegmentsResponse> {
        let span = span!(Level::INFO, "get_segment");
        let _guard = span.enter();

        let path = &format!("/segments/{segment_id}");
        let res = make_request(self.token, Method::GET, path, None).await?;
        if !matches!(res.status(), StatusCode::OK) {
            return Err(async_graphql::Error::new(res.text().await?));
        }
        let text = res.text().await?;
        info!("text response: `{}`", text);
        Ok(serde_json::from_str(&text)?)
    }

    // Update a segment
    pub async fn update(&self, segment: UpdateSegmentRequest) -> Result<SegmentsResponse> {
        let span = span!(Level::INFO, "update_segment");
        let _guard = span.enter();

        let path = &format!("/adaccounts/{}/segments", segment.ad_account_id);
        let body = serde_json::to_string(&UpdateSegmentsRequest {
            segments: vec![segment],
        })?;
        info!("reqeust body: `{}`", body);
        let res = make_request(self.token, Method::PUT, path, Some(body)).await?;
        if !matches!(res.status(), StatusCode::OK) {
            return Err(async_graphql::Error::new(res.text().await?));
        }
        let text = res.text().await?;
        info!("text response: `{}`", text);
        Ok(serde_json::from_str(&text)?)
    }

    // Delete a segment
    pub async fn delete(&self, segment_id: String) -> Result<SegmentsResponse> {
        let span = span!(Level::INFO, "delete_segment");
        let _guard = span.enter();

        let path = &format!("/segments/{segment_id}");
        let res = make_request(self.token, Method::DELETE, path, None).await?;
        if !matches!(res.status(), StatusCode::OK) {
            return Err(async_graphql::Error::new(res.text().await?));
        }
        let text = res.text().await?;
        info!("text response: `{}`", text);
        Ok(serde_json::from_str(&text)?)
    }
}
