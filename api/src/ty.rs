use async_graphql::{Context, Result};

pub struct AppInfo {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: String,
    pub refresh_token: String,
    pub ad_account_id: String,
}

pub fn get_app_info<'a>(ctx: &'a Context<'_>) -> Result<&'a AppInfo> {
    match ctx.data::<AppInfo>() {
        Ok(info) => Ok(info),
        Err(err) => Err(format!("app information not found: {}", err.message).into()),
    }
}
