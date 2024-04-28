use async_graphql::{Context, Error, Object};
use async_trait::async_trait;

use snapchat::client::SnapChat;
use snapchat::ty::segment::SegmentsResponse;

use super::ty::get_app_info;

pub struct RootQuery;

#[async_trait]
trait Queries {
    async fn get_segments(
        &self,
        ctx: &Context<'_>,
        token: String,
        ad_account_id: String,
        refresh_token: String,
    ) -> Result<SegmentsResponse, Error>;
}

#[Object]
#[async_trait]
impl Queries for RootQuery {
    async fn get_segments(
        &self,
        ctx: &Context<'_>,
        token: String,
        ad_account_id: String,
        refresh_token: String,
    ) -> Result<SegmentsResponse, Error> {
        let app = get_app_info(ctx)?;
        let snapchat = SnapChat::new(
            &token,
            &app.client_id,
            &app.client_secret,
            &app.redirect_uri,
            &refresh_token,
        )
        .await;
        let segment_client = snapchat.segment().await;

        Ok(segment_client.get_all(ad_account_id).await?)
    }
}
