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
  pub user_id: String,
  pub basic_id: String,
  pub display_name: String,
  pub picture_url: String,
  pub chat_mode: String,
  pub mark_as_read_mode: String,
}
