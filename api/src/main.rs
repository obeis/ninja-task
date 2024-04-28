use std::sync::RwLock;
use std::{env, sync::Arc};

use async_graphql::{EmptyMutation, EmptySubscription, Result, Schema};
use poem::middleware::AddData;
use poem::post;
use poem::{get, listener::TcpListener, EndpointExt, Route, Server};
use tracing::{info, span, Level};

use handler::{graphql, oauth2_code};
use query::RootQuery;

mod handler;
mod query;

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

    let schema = Schema::build(RootQuery, EmptyMutation, EmptySubscription).finish();

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
