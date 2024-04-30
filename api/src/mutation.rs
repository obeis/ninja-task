use async_graphql::{Context, Error, Object};
use async_trait::async_trait;

use snapchat::client::SnapChat;
use snapchat::ty::segment::SegmentsResponse;

use super::ty::get_app_info;

pub struct RootMutation;

#[async_trait]
trait Mutations {
    async fn delete_all_users(
        &self,
        ctx: &Context<'_>,
        segment_id: String,
    ) -> Result<SegmentsResponse, Error>;
}

#[Object]
#[async_trait]
impl Mutations for RootMutation {
    async fn delete_all_users(
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
        let user_client = snapchat.user().await;

        Ok(user_client.delete_all_users(segment_id).await?)
    }
}
