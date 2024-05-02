use std::fmt::{Display, Formatter, Result};
use std::result::Result as FromStrResult;
use std::str::FromStr;

use anyhow::bail;
use async_graphql::{Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct SegmentsRequest {
    pub segments: Vec<SegmentRequest>,
}

impl Display for SegmentsRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[")?;
        for (i, s) in self.segments.iter().enumerate() {
            if i > 0 {
                write!(f, " , ")?;
            }
            write!(f, "{{")?;
            write!(f, " name: \\\"{}\\\",", s.name)?;
            write!(f, " description: \\\"{}\\\",", s.description)?;
            write!(f, " source_type: \\\"{}\\\",", s.source_type)?;
            write!(f, " retention_in_days: {},", s.retention_in_days)?;
            write!(f, " ad_account_id: \\\"{}\\\"", s.ad_account_id)?;
            write!(f, "}}")?;
        }
        write!(f, "]")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(rename_fields = "snake_case")]
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
            DataSourceType::FirstParty => write!(f, "FIRST_PARTY"),
            DataSourceType::Engagement => write!(f, "ENGAGEMENT"),
            DataSourceType::Pixel => write!(f, "PIXEL"),
            DataSourceType::Mobile => write!(f, "MOBILE"),
            DataSourceType::FootTrafficInsights => write!(f, "FOOT_TRAFFIC_INSIGHTS"),
        }
    }
}

impl FromStr for DataSourceType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> FromStrResult<Self, Self::Err> {
        match s {
            "FIRST_PARTY" => Ok(DataSourceType::FirstParty),
            "ENGAGEMENT" => Ok(DataSourceType::Engagement),
            "PIXEL" => Ok(DataSourceType::Pixel),
            "MOBILE" => Ok(DataSourceType::Mobile),
            "FOOT_TRAFFIC_INSIGHTS" => Ok(DataSourceType::FootTrafficInsights),
            _ => bail!("unknown `DataSourceType`: {}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct SegmentsResponse {
    pub request_status: String,
    pub request_id: String,
    pub segments: Vec<SegmentResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct SegmentResponse {
    pub sub_request_status: String,
    pub segment: Segment,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
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

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(rename_fields = "snake_case")]
pub struct UpdateSegmentRequest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub retention_in_days: u32,
    pub ad_account_id: String,
}

impl Display for UpdateSegmentRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{{")?;
        write!(f, " id: \\\"{}\\\",", self.id)?;
        write!(f, " name: \\\"{}\\\",", self.name)?;
        write!(f, " description: \\\"{}\\\",", self.description)?;
        write!(f, " retention_in_days: {},", self.retention_in_days)?;
        write!(f, " ad_account_id: \\\"{}\\\"", self.ad_account_id)?;
        write!(f, "}}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
#[graphql(rename_fields = "snake_case")]
pub struct UpdateSegmentsRequest {
    pub segments: Vec<UpdateSegmentRequest>,
}
