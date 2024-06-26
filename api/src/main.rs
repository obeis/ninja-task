use std::sync::RwLock;
use std::{env, sync::Arc};

use async_graphql::{EmptySubscription, Result, Schema};
use poem::middleware::{AddData, Cors};
use poem::post;
use poem::{get, listener::TcpListener, EndpointExt, Route, Server};
use tokio::sync::Mutex;
use tracing::{info, span, Level};

use handler::{graphql, oauth2_code};
use mutation::RootMutation;
use query::RootQuery;
use ty::AppInfo;

mod handler;
mod mutation;
mod query;
mod ty;

struct Configuration {
    wvt: RwLock<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let span = span!(Level::INFO, "main");
    let _guard = span.enter();

    let wvt = env::var("WEBHOOK_VERIFICATION_TOKEN")?;

    let config = Arc::new(Configuration {
        wvt: RwLock::new(wvt),
    });

    let app_info = AppInfo {
        client_id: Arc::new(Mutex::new(env::var("SNAPCHAT_CLIENT_ID")?)),
        client_secret: Arc::new(Mutex::new(env::var("SNAPCHAT_CLIENT_SECRET")?)),
        access_token: Arc::new(Mutex::new(env::var("SNAPCHAT_ACCESS_TOKEN")?)),
        refresh_token: Arc::new(Mutex::new(env::var("SNAPCHAT_REFRESH_TOKEN")?)),
        ad_account_id: Arc::new(Mutex::new(env::var("SNAPCHAT_AD_ACCOUNT_ID")?)),
    };

    let schema = Schema::build(RootQuery, RootMutation, EmptySubscription)
        .data(app_info)
        .finish();

    let app = Route::new()
        .at("/auth/:wvt", get(oauth2_code))
        .at("/graphql", post(graphql).data(schema))
        .with(AddData::new(config))
        .with(Cors::new());

    info!("server is running on :4000");

    Server::new(TcpListener::bind("0.0.0.0:4000"))
        .run(app)
        .await?;

    Ok(())
}
