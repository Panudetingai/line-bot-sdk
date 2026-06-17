use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[napi(object)]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
  pub display_name: String,
  pub user_id: String,
  pub picture_url: Option<String>,
  pub status_message: Option<String>,
}

#[napi(object)]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BotInfo {
  pub userId: String,
  pub basicId: String,
  pub displayName: String,
  pub pictureUrl: String,
  pub chatMode: String,
  pub markAsReadMode: String,
}
