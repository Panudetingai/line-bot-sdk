use napi_derive::napi;
use serde::{Deserialize, Serialize};

// ── Text ─────────────────────────────────────────────────────────────────────

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextMessage {
  #[serde(rename = "type")]
  pub message_type: String,
  pub text: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub quick_reply: Option<QuickReply>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sender: Option<Sender>,
}

impl TextMessage {
  pub fn new(text: String) -> Self {
    Self { message_type: "text".into(), text, quick_reply: None, sender: None }
  }
}

// ── Image ────────────────────────────────────────────────────────────────────

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageMessage {
  #[serde(rename = "type")]
  pub message_type: String,
  pub original_content_url: String,
  pub preview_image_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub quick_reply: Option<QuickReply>,
}

impl ImageMessage {
  pub fn new(original_content_url: String, preview_image_url: String) -> Self {
    Self {
      message_type: "image".into(),
      original_content_url,
      preview_image_url,
      quick_reply: None,
    }
  }
}

// ── Video ────────────────────────────────────────────────────────────────────

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoMessage {
  #[serde(rename = "type")]
  pub message_type: String,
  pub original_content_url: String,
  pub preview_image_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tracking_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub quick_reply: Option<QuickReply>,
}

impl VideoMessage {
  pub fn new(original_content_url: String, preview_image_url: String) -> Self {
    Self {
      message_type: "video".into(),
      original_content_url,
      preview_image_url,
      tracking_id: None,
      quick_reply: None,
    }
  }
}

// ── Audio ────────────────────────────────────────────────────────────────────

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioMessage {
  #[serde(rename = "type")]
  pub message_type: String,
  pub original_content_url: String,
  pub duration: i32,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub quick_reply: Option<QuickReply>,
}

// ── Location ─────────────────────────────────────────────────────────────────

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationMessage {
  #[serde(rename = "type")]
  pub message_type: String,
  pub title: String,
  pub address: String,
  pub latitude: f64,
  pub longitude: f64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub quick_reply: Option<QuickReply>,
}

// ── Sticker ──────────────────────────────────────────────────────────────────

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickerMessage {
  #[serde(rename = "type")]
  pub message_type: String,
  pub package_id: String,
  pub sticker_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub quick_reply: Option<QuickReply>,
}

// ── Quick Reply ───────────────────────────────────────────────────────────────

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickReply {
  pub items: Vec<QuickReplyItem>,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickReplyItem {
  #[serde(rename = "type")]
  pub item_type: String,
  pub action: QuickReplyAction,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub image_url: Option<String>,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickReplyAction {
  #[serde(rename = "type")]
  pub action_type: String,
  pub label: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub text: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub uri: Option<String>,
}

// ── Sender ───────────────────────────────────────────────────────────────────

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sender {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub icon_url: Option<String>,
}

// ── Request payloads (kept for backward compat) ───────────────────────────────

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushMessageRequest {
  pub to: String,
  pub messages: Vec<TextMessage>,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplyMessageRequest {
  pub reply_token: String,
  pub messages: Vec<TextMessage>,
}

// ── Quota ────────────────────────────────────────────────────────────────────

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageQuota {
  #[serde(rename = "type")]
  pub quota_type: String,
  pub value: Option<i64>,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageQuotaConsumption {
  pub total_usage: i64,
}
