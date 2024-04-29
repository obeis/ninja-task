use std::sync::RwLock;
use std::{env, sync::Arc};

use async_graphql::{EmptyMutation, EmptySubscription, Result, Schema};
use poem::middleware::AddData;
use poem::post;
use poem::{get, listener::TcpListener, EndpointExt, Route, Server};
use tracing::{info, span, Level};

use handler::{graphql, oauth2_code};
use query::RootQuery;
use ty::AppInfo;

mod handler;
mod query;
mod ty;

struct Configuration {
    wvt: RwLock<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let span = span!(Level::INFO, "main");
    let _guard = span.enter();

    let wvt = env::var("WEBHOOK_VERIFICATION_TOKEN")?;

    let config = Arc::new(Configuration {
        wvt: RwLock::new(wvt),
    });

    let app_info = AppInfo {
        client_id: env::var("SNAPCHAT_CLIENT_ID")?,
        client_secret: env::var("SNAPCHAT_CLIENT_SECRET")?,
        access_token: env::var("SNAPCHAT_ACCESS_TOKN")?,
        refresh_token: env::var("SNAPCHAT_REFRESH_TOKEN")?,
        ad_account_id: env::var("SNAPCHAT_AD_ACCOUNT_ID")?,
    };

    let schema = Schema::build(RootQuery, EmptyMutation, EmptySubscription)
        .data(app_info)
        .finish();

    let app = Route::new()
        .at("/auth/:wvt", get(oauth2_code))
        .at("/graphql", post(graphql).data(schema))
        .with(AddData::new(config));

    info!("server is running on :4000");

    Server::new(TcpListener::bind("0.0.0.0:4000"))
        .run(app)
        .await?;

    Ok(())
}
