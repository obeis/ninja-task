use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    access_token: String,
    token_type: String,
    expires_in: u32,
    refresh_token: String,
    scope: String,
}
