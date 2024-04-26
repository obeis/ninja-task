use std::env;

use async_graphql::Result;
use snapchat::{client::SnapChat, ty::SegmentRequest};

mod snapchat;

#[tokio::main]
async fn main() -> Result<()> {
    let token = env::var("SNAPCHAT_TOKEN")?;
    let ad_account_id = env::var("SNAPCHAT_AD_ACCOUNT_ID")?;

    let client = SnapChat::new(token).await;
    let seg1 = SegmentRequest {
        name: "Seg 1".to_string(),
        description: "desc 1".to_string(),
        source_type: snapchat::ty::DataSourceType::FirstParty,
        retention_in_days: 1,
        ad_account_id: ad_account_id.clone(),
    };
    let seg2 = SegmentRequest {
        name: "Seg 2".to_string(),
        description: "desc 2".to_string(),
        source_type: snapchat::ty::DataSourceType::FirstParty,
        retention_in_days: 1,
        ad_account_id,
    };
    let segments = vec![seg1, seg2];
    let res = client.segment().await.create(segments).await?;
    println!("response: {:#?}", res);

    Ok(())
}
