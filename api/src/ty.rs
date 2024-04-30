use std::sync::Arc;

use async_graphql::{Context, Result};
use tokio::sync::Mutex;

pub struct AppInfo {
    pub client_id: Arc<Mutex<String>>,
    pub client_secret: Arc<Mutex<String>>,
    pub access_token: Arc<Mutex<String>>,
    pub refresh_token: Arc<Mutex<String>>,
    pub ad_account_id: Arc<Mutex<String>>,
}

pub fn get_app_info<'a>(ctx: &'a Context<'_>) -> Result<&'a AppInfo> {
    match ctx.data::<AppInfo>() {
        Ok(info) => Ok(info),
        Err(err) => Err(format!("app information not found: {}", err.message).into()),
    }
}
