use std::env;

use async_graphql::Result;
use snapchat::client::SnapChat;

mod snapchat;

#[tokio::main]
async fn main() -> Result<()> {
    let token = env::var("SNAPCHAT_TOKEN")?;
    let ad_account_id = env::var("SNAPCHAT_AD_ACCOUNT_ID")?;

    let client = SnapChat::new(token).await;
    let res = client
        .segment()
        .await
        .update(snapchat::ty::UpdateSegmentRequest {
            id: "6798030683466871".to_string(),
            name: "Seg 2 Update".to_string(),
            description: "desc 22".to_string(),
            retention_in_days: 5,
            ad_account_id,
        })
        .await?;
    println!("response: {:#?}", res);

    Ok(())
}
