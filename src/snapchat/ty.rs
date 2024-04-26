use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SegmentsRequest {
    pub segments: Vec<SegmentRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SegmentRequest {
    pub name: String,
    pub description: String,
    pub source_type: DataSourceType,
    pub retention_in_days: u32,
    pub ad_account_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataSourceType {
    FirstParty,
    Engagement,
    Pixel,
    Mobile,
    FootTrafficInsights,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SegmentsResponse {
    pub request_status: String,
    pub request_id: String,
    pub segments: Vec<SegmentResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SegmentResponse {
    pub sub_request_status: String,
    pub segment: Segment,
}

#[derive(Debug, Serialize, Deserialize)]
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
