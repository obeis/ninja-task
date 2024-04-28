use std::sync::RwLock;
use std::{env, sync::Arc};

use async_graphql::Result;
use poem::middleware::AddData;
use poem::{get, listener::TcpListener, EndpointExt, Route, Server};

use handler::oauth2_code;

mod handler;

struct Configuration {
    wvt: RwLock<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let wvt = env::var("WEBHOOK_VERIFICATION_TOKEN")?;

    let config = Arc::new(Configuration {
        wvt: RwLock::new(wvt),
    });

    let app = Route::new()
        .at("/auth/:wvt", get(oauth2_code))
        .with(AddData::new(config));

    Server::new(TcpListener::bind("0.0.0.0:4000"))
        .run(app)
        .await?;

    Ok(())
}
