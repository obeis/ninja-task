use poem::{get, handler, listener::TcpListener, web::Redirect, Request, Result, Route, Server};
use serde::Deserialize;

#[derive(Deserialize)]
struct Params {
    code: String,
}

#[handler]
fn oauth2_code(req: &Request) -> Result<()> {
    let params = req.params::<Params>()?;

    Redirect::moved_permanent(format!("https://khwarizmi.io/auth/{}", params.code));

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/auth", get(oauth2_code));

    Server::new(TcpListener::bind("0.0.0.0:4000"))
        .run(app)
        .await
}
