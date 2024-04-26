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
        .get("6798030683466871".to_string())
        .await?;
    println!("response: {:#?}", res);

    Ok(())
}
