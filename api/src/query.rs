use async_graphql::{Context, Error, Object};
use async_trait::async_trait;

use snapchat::client::SnapChat;
use snapchat::ty::segment::SegmentsResponse;

pub struct RootQuery;

#[async_trait]
trait Queries {
    async fn get_segments(
        &self,
        ctx: &Context<'_>,
        token: String,
        ad_account_id: String,
    ) -> Result<SegmentsResponse, Error>;
}

#[Object]
#[async_trait]
impl Queries for RootQuery {
    async fn get_segments(
        &self,
        _ctx: &Context<'_>,
        token: String,
        ad_account_id: String,
    ) -> Result<SegmentsResponse, Error> {
        let snapchat = SnapChat::new(token).await;
        let segment_client = snapchat.segment().await;

        Ok(segment_client.get_all(ad_account_id).await?)
    }
}
