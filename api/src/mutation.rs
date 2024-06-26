use async_graphql::{Context, Error, Object};
use async_trait::async_trait;

use snapchat::client::SnapChat;
use snapchat::ty::segment::{SegmentRequest, SegmentsResponse, UpdateSegmentRequest};
use snapchat::ty::user::UsersResponse;
use snapchat::user::UserIdentifier;

use super::ty::get_app_info;

pub struct RootMutation;

#[async_trait]
trait Mutations {
    async fn create_segments(
        &self,
        ctx: &Context<'_>,
        segements: Vec<SegmentRequest>,
    ) -> Result<SegmentsResponse, Error>;

    async fn update_segment(
        &self,
        ctx: &Context<'_>,
        segment: UpdateSegmentRequest,
    ) -> Result<SegmentsResponse, Error>;

    async fn delete_segment(
        &self,
        ctx: &Context<'_>,
        segment_id: String,
    ) -> Result<SegmentsResponse, Error>;

    async fn add_users(
        &self,
        ctx: &Context<'_>,
        segment_id: String,
        identifiers: Vec<String>,
        schema_ty: String,
    ) -> Result<UsersResponse, Error>;

    async fn delete_users(
        &self,
        ctx: &Context<'_>,
        segment_id: String,
        identifiers: Vec<String>,
        schema_ty: String,
    ) -> Result<UsersResponse, Error>;

    async fn delete_all_users(
        &self,
        ctx: &Context<'_>,
        segment_id: String,
    ) -> Result<SegmentsResponse, Error>;
}

#[Object]
#[async_trait]
impl Mutations for RootMutation {
    async fn create_segments(
        &self,
        ctx: &Context<'_>,
        mut segments: Vec<SegmentRequest>,
    ) -> Result<SegmentsResponse, Error> {
        let app = get_app_info(ctx)?;
        let access_token = app.access_token.lock().await;
        let client_id = app.client_id.lock().await;
        let client_secret = app.client_secret.lock().await;
        let refresh_token = app.refresh_token.lock().await;
        let ad_account_id = app.ad_account_id.lock().await;
        let snapchat =
            SnapChat::new(&access_token, &client_id, &client_secret, &refresh_token).await;
        let segment_client = snapchat.segment().await;

        for seg in segments.iter_mut() {
            seg.ad_account_id = ad_account_id.to_string();
        }

        Ok(segment_client.create(segments).await?)
    }

    async fn update_segment(
        &self,
        ctx: &Context<'_>,
        mut segment: UpdateSegmentRequest,
    ) -> Result<SegmentsResponse, Error> {
        let app = get_app_info(ctx)?;
        let access_token = app.access_token.lock().await;
        let client_id = app.client_id.lock().await;
        let client_secret = app.client_secret.lock().await;
        let refresh_token = app.refresh_token.lock().await;
        let ad_account_id = app.ad_account_id.lock().await;
        let snapchat =
            SnapChat::new(&access_token, &client_id, &client_secret, &refresh_token).await;
        let segment_client = snapchat.segment().await;
        segment.ad_account_id = ad_account_id.to_string();
        Ok(segment_client.update(segment).await?)
    }

    async fn delete_segment(
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
        Ok(segment_client.delete(segment_id).await?)
    }

    async fn add_users(
        &self,
        ctx: &Context<'_>,
        segment_id: String,
        identifiers: Vec<String>,
        schema_ty: String,
    ) -> Result<UsersResponse, Error> {
        let app = get_app_info(ctx)?;
        let access_token = app.access_token.lock().await;
        let client_id = app.client_id.lock().await;
        let client_secret = app.client_secret.lock().await;
        let refresh_token = app.refresh_token.lock().await;
        let snapchat =
            SnapChat::new(&access_token, &client_id, &client_secret, &refresh_token).await;
        let user_client = snapchat.user().await;

        Ok(user_client
            .add_users(
                segment_id,
                identifiers,
                UserIdentifier::from_string(schema_ty),
            )
            .await?)
    }

    async fn delete_users(
        &self,
        ctx: &Context<'_>,
        segment_id: String,
        identifiers: Vec<String>,
        schema_ty: String,
    ) -> Result<UsersResponse, Error> {
        let app = get_app_info(ctx)?;
        let access_token = app.access_token.lock().await;
        let client_id = app.client_id.lock().await;
        let client_secret = app.client_secret.lock().await;
        let refresh_token = app.refresh_token.lock().await;
        let snapchat =
            SnapChat::new(&access_token, &client_id, &client_secret, &refresh_token).await;
        let user_client = snapchat.user().await;

        Ok(user_client
            .delete_users(
                segment_id,
                identifiers,
                UserIdentifier::from_string(schema_ty),
            )
            .await?)
    }

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
