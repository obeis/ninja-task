use async_graphql::{Context, Error, Object};
use async_trait::async_trait;

use snapchat::client::SnapChat;
use snapchat::ty::auth::Token;
use snapchat::ty::segment::SegmentsResponse;

use super::ty::get_app_info;

pub struct RootQuery;

#[async_trait]
trait Queries {
    async fn generate_token(&self, ctx: &Context<'_>, code: String) -> Result<Token, Error>;

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
    async fn generate_token(&self, ctx: &Context<'_>, code: String) -> Result<Token, Error> {
        let app = get_app_info(ctx)?;
        let snapchat = SnapChat::new(
            "",
            &app.client_id,
            &app.client_secret,
            &app.redirect_uri,
            "",
        )
        .await;
        let auth_client = snapchat.auth().await;
        Ok(auth_client.generate_token(&code).await?)
    }

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
