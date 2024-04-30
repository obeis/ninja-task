use std::fmt::{Display, Formatter, Result};

use async_graphql::{Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct SegmentsRequest {
    pub segments: Vec<SegmentRequest>,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(rename_fields = "snake_case")]
#[graphql(input_name = "segment_input")]
pub struct SegmentRequest {
    pub name: String,
    pub description: String,
    pub source_type: DataSourceType,
    pub retention_in_days: u32,
    pub ad_account_id: String,
}

#[derive(Debug, Serialize, Deserialize, Enum, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataSourceType {
    FirstParty,
    Engagement,
    Pixel,
    Mobile,
    FootTrafficInsights,
}

impl Display for DataSourceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            DataSourceType::FirstParty => write!(f, "First Party"),
            DataSourceType::Engagement => write!(f, "Engagement"),
            DataSourceType::Pixel => write!(f, "Pixel"),
            DataSourceType::Mobile => write!(f, "Mobile"),
            DataSourceType::FootTrafficInsights => write!(f, "Foot Traffic Insights"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct SegmentsResponse {
    pub request_status: String,
    pub request_id: String,
    pub segments: Vec<SegmentResponse>,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct SegmentResponse {
    pub sub_request_status: String,
    pub segment: Segment,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct Segment {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub source_type: DataSourceType,
    pub ad_account_id: String,
    pub organization_id: String,
    pub targetable_status: String,
    pub upload_status: String,
    pub retention_in_days: u32,
    pub approximate_number_users: u32,
    pub visible_to: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct UpdateSegmentRequest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub retention_in_days: u32,
    pub ad_account_id: String,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct UpdateSegmentsRequest {
    pub segments: Vec<UpdateSegmentRequest>,
}
