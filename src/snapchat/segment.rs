use async_graphql::{Error, Result};
use reqwest::{Method, StatusCode};

use super::{
    request::make_request,
    ty::{
        SegmentRequest, SegmentsRequest, SegmentsResponse, UpdateSegmentRequest,
        UpdateSegmentsRequest,
    },
};

pub(crate) struct SegmentService<'a> {
    token: &'a str,
}

impl<'a> SegmentService<'a> {
    pub async fn new(token: &'a str) -> Self {
        Self { token }
    }

    pub async fn create(&self, segments: Vec<SegmentRequest>) -> Result<SegmentsResponse> {
        let ad_account_id = if let Some(seg) = segments.last() {
            &seg.ad_account_id
        } else {
            return Err(Error::new("empty segment list"));
        };
        let path = &format!("/adaccounts/{}/segments", ad_account_id);
        let body = serde_json::to_string(&SegmentsRequest { segments })?;
        let res = make_request(self.token, Method::POST, path, Some(body)).await?;
        if !matches!(res.status(), StatusCode::OK) {
            return Err(async_graphql::Error::new(res.text().await?));
        }
        Ok(res.json().await?)
    }

    pub async fn get_all(&self, ad_account_id: String) -> Result<SegmentsResponse> {
        let path = &format!("/adaccounts/{ad_account_id}/segments");
        let res = make_request(self.token, Method::GET, path, None).await?;
        if !matches!(res.status(), StatusCode::OK) {
            return Err(async_graphql::Error::new(res.text().await?));
        }
        Ok(res.json().await?)
    }

    pub async fn get(&self, segment_id: String) -> Result<SegmentsResponse> {
        let path = &format!("/segments/{segment_id}");
        let res = make_request(self.token, Method::GET, path, None).await?;
        if !matches!(res.status(), StatusCode::OK) {
            return Err(async_graphql::Error::new(res.text().await?));
        }
        Ok(res.json().await?)
    }

    pub async fn update(&self, segment: UpdateSegmentRequest) -> Result<SegmentsResponse> {
        let path = &format!("/adaccounts/{}/segments", segment.ad_account_id);
        let body = serde_json::to_string(&UpdateSegmentsRequest {
            segments: vec![segment],
        })?;
        let res = make_request(self.token, Method::PUT, path, Some(body)).await?;
        if !matches!(res.status(), StatusCode::OK) {
            return Err(async_graphql::Error::new(res.text().await?));
        }
        Ok(res.json().await?)
    }
}
