use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichMenuSize {
  pub width: i32,
  pub height: i32,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichMenuBounds {
  pub x: i32,
  pub y: i32,
  pub width: i32,
  pub height: i32,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichMenuAction {
  #[serde(rename = "type")]
  pub action_type: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub text: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub uri: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<String>,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichMenuArea {
  pub bounds: RichMenuBounds,
  pub action: RichMenuAction,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichMenu {
  pub size: RichMenuSize,
  pub selected: bool,
  pub name: String,
  pub chat_bar_text: String,
  pub areas: Vec<RichMenuArea>,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichMenuResponse {
  pub rich_menu_id: String,
  pub size: RichMenuSize,
  pub selected: bool,
  pub name: String,
  pub chat_bar_text: String,
  pub areas: Vec<RichMenuArea>,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichMenuIdResponse {
  #[serde(rename = "richMenuId")]
  pub rich_menu_id: String,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichMenuList {
  pub richmenus: Vec<RichMenuResponse>,
}
