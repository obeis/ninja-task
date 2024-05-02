use std::env::var;

use anyhow::Result;
use poem::{endpoint::StaticFilesEndpoint, listener::TcpListener, Route, Server};

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

    let app = Route::new().nest(
        "/",
        StaticFilesEndpoint::new(var("WEB_FILES_PATH")?)
            .show_files_listing()
            .index_file("index.html"),
    );

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await?;

    Ok(())
}
