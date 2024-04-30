use std::sync::Arc;

use async_graphql::{EmptySubscription, Schema};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
use poem::{
    error::ResponseError,
    handler,
    http::StatusCode,
    web::{Data, Path, Redirect},
    Request, Result,
};
use serde::Deserialize;

use crate::Configuration;
use crate::{mutation::RootMutation, query::RootQuery};

#[derive(Debug, thiserror::Error)]
#[error("invalid webhook verification token")]
struct InvalidWvt;

impl ResponseError for InvalidWvt {
    fn status(&self) -> StatusCode {
        StatusCode::NON_AUTHORITATIVE_INFORMATION
    }
}

#[derive(Deserialize)]
struct Params {
    code: String,
}

#[handler]
pub fn oauth2_code(
    req: &Request,
    Path(wvt): Path<String>,
    config: Data<&Arc<Configuration>>,
) -> Result<()> {
    let webhook_verification_token = config.wvt.read().unwrap();
    if wvt != *webhook_verification_token {
        return Err(InvalidWvt.into());
    }

    let params = req.params::<Params>()?;

    Redirect::moved_permanent(format!("https://khwarizmi.io/auth/{}", params.code));

    Ok(())
}

pub type ApiSchema = Schema<RootQuery, RootMutation, EmptySubscription>;

#[handler]
pub async fn graphql(schema: Data<&ApiSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.0).await.into()
}
