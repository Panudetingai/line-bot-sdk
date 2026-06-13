use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageDeliveryOverview {
  pub status: String,
  pub broadcast: Option<i64>,
  pub targeting: Option<i64>,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FollowersInsight {
  pub status: String,
  pub followers: Option<i64>,
  pub targeted_reaches: Option<i64>,
  pub blocks: Option<i64>,
}
