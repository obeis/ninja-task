use anyhow::Result;
use reqwest::Response;
use serde_json::Value;
use web_sys::console;

use snapchat::ty::segment::SegmentResponse;

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

const GET_SEGMENTS_QUERY: &str = r#"{"query":"query {\ngetSegments {\nsegments {\nsubRequestStatus\nsegment {\nid\nname\ndescription\nstatus\nsourceType\nadAccountId\norganizationId\ntargetableStatus\nuploadStatus\nretentionInDays\napproximateNumberUsers\nvisibleTo\ncreatedAt\nupdatedAt\n}\n}\n}\n}","variables":{}}"#;

pub async fn get_segments() -> Result<Vec<SegmentResponse>> {
    let res = fetch(GET_SEGMENTS_QUERY.to_string()).await?;

    let root: Value = serde_json::from_str(&res.text().await?)?;
    console::log_1(&GET_SEGMENTS_QUERY.to_string().into());
    let payload = root
        .get("data")
        .and_then(|value| value.get("getSegments"))
        .and_then(|value| value.get("segments"));
    console::log_1(&payload.unwrap().to_string().into());

    Ok(serde_json::from_str(&payload.unwrap().to_string())?)
}
