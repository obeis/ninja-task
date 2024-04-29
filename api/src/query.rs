use async_graphql::{Context, Error, Object};
use async_trait::async_trait;

use snapchat::client::SnapChat;
use snapchat::ty::auth::Token;
use snapchat::ty::segment::SegmentsResponse;

use super::ty::get_app_info;

pub struct RootQuery;

#[async_trait]
trait Queries {
    async fn refresh_token(
        &self,
        ctx: &Context<'_>,
        rerefresh_token: String,
    ) -> Result<Token, Error>;

    async fn get_segments(&self, ctx: &Context<'_>) -> Result<SegmentsResponse, Error>;
}

#[Object]
#[async_trait]
impl Queries for RootQuery {
    async fn refresh_token(
        &self,
        ctx: &Context<'_>,
        refresh_token: String,
    ) -> Result<Token, Error> {
        let app = get_app_info(ctx)?;
        let snapchat = SnapChat::new("", &app.client_id, &app.client_secret, &refresh_token).await;
        let auth_client = snapchat.auth().await;
        Ok(auth_client.refresh_token().await?)
    }

    async fn get_segments(&self, ctx: &Context<'_>) -> Result<SegmentsResponse, Error> {
        let app = get_app_info(ctx)?;
        let snapchat = SnapChat::new(
            &app.access_token,
            &app.client_id,
            &app.client_secret,
            &app.refresh_token,
        )
        .await;
        let segment_client = snapchat.segment().await;

        Ok(segment_client.get_all(&app.ad_account_id).await?)
    }
}
