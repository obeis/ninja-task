use std::env;

use async_graphql::Result;
use snapchat::client::SnapChat;

mod snapchat;

#[tokio::main]
async fn main() -> Result<()> {
    let token = env::var("SNAPCHAT_TOKEN")?;
    let _ad_account_id = env::var("SNAPCHAT_AD_ACCOUNT_ID")?;

    let client = SnapChat::new(token).await;
    let res = client
        .user()
        .await
        .add_users(
            "6798030683466871".to_string(),
            vec!["g1@gmail.com".to_string(), "g2@gmail.com".to_string()],
        )
        .await?;

    println!("response: {:#?}", res);

    Ok(())
}
