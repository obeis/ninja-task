use async_graphql::{Context, Error, Object};
use async_trait::async_trait;

use snapchat::client::SnapChat;
use snapchat::ty::auth::Token;
use snapchat::ty::segment::SegmentsResponse;

use super::ty::get_app_info;

pub struct RootQuery;

#[async_trait]
trait Queries {
    async fn refresh_token(&self, ctx: &Context<'_>) -> Result<Token, Error>;

    async fn get_segments(&self, ctx: &Context<'_>) -> Result<SegmentsResponse, Error>;

    async fn get_segment(
        &self,
        ctx: &Context<'_>,
        segment_id: String,
    ) -> Result<SegmentsResponse, Error>;
}

#[Object]
#[async_trait]
impl Queries for RootQuery {
    async fn refresh_token(&self, ctx: &Context<'_>) -> Result<Token, Error> {
        let app = get_app_info(ctx)?;
        let client_id = app.client_id.lock().await;
        let client_secret = app.client_secret.lock().await;
        let mut refresh_token = app.refresh_token.lock().await;
        let snapchat = SnapChat::new("", &client_id, &client_secret, &refresh_token).await;
        let auth_client = snapchat.auth().await;
        let token = auth_client.refresh_token().await?;

        let mut access_token = app.access_token.lock().await;
        access_token.clone_from(&token.access_token);
        refresh_token.clone_from(&token.refresh_token);

        Ok(token)
    }

    async fn get_segments(&self, ctx: &Context<'_>) -> Result<SegmentsResponse, Error> {
        let app = get_app_info(ctx)?;
        let access_token = app.access_token.lock().await;
        let client_id = app.client_id.lock().await;
        let client_secret = app.client_secret.lock().await;
        let refresh_token = app.refresh_token.lock().await;
        let snapchat =
            SnapChat::new(&access_token, &client_id, &client_secret, &refresh_token).await;
        let segment_client = snapchat.segment().await;

        let ad_account_id = app.ad_account_id.lock().await;
        Ok(segment_client.get_all(&ad_account_id).await?)
    }

    async fn get_segment(
        &self,
        ctx: &Context<'_>,
        segment_id: String,
    ) -> Result<SegmentsResponse, Error> {
        let app = get_app_info(ctx)?;
        let access_token = app.access_token.lock().await;
        let client_id = app.client_id.lock().await;
        let client_secret = app.client_secret.lock().await;
        let refresh_token = app.refresh_token.lock().await;
        let snapchat =
            SnapChat::new(&access_token, &client_id, &client_secret, &refresh_token).await;
        let segment_client = snapchat.segment().await;

        Ok(segment_client.get(segment_id).await?)
    }
}
