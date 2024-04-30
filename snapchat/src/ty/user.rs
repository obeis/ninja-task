use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct UsersRequest {
    pub users: Vec<UserRequest>,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct UserRequest {
    pub schema: Vec<String>,
    pub data: Vec<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct UsersResponse {
    pub request_status: String,
    pub request_id: String,
    pub users: Vec<UserResponse>,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct UserResponse {
    pub sub_request_status: String,
    pub user: SubUserResponse,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct SubUserResponse {
    pub number_uploaded_users: u32,
}
