use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupSummary {
  pub group_id: String,
  pub group_name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub picture_url: Option<String>,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMemberProfile {
  pub display_name: String,
  pub user_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub picture_url: Option<String>,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberIds {
  pub member_ids: Vec<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub next: Option<String>,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowerIds {
  pub user_ids: Vec<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub next: Option<String>,
}
