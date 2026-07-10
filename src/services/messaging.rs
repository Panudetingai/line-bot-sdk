use crate::api::endpoints;
use crate::client::LineClient;
use crate::types::message::{
  MessageQuota, MessageQuotaConsumption, PushMessageRequest, ReplyMessageRequest, TextMessage,
};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde_json::Value;

#[napi]
impl LineClient {
  // ── Text convenience ─────────────────────────────────────────────────────────

  #[napi]
  pub async fn push_message(&self, to: String, text: String) -> Result<String> {
    let body = PushMessageRequest {
      to,
      messages: vec![TextMessage::new(text)],
    };
    self
      .request_post_json(&endpoints::push_message(), &body)
      .await
  }

  #[napi]
  pub async fn narrowcast_message(&self, messages_json: String) -> Result<String> {
    let messages: Vec<Value> = serde_json::from_str(&messages_json)
      .map_err(|e| Error::from_reason(format!("Invalid messages JSON: {e}")))?;
    let body = serde_json::json!({ "messages": messages });
    self
      .request_post_json(&endpoints::narrowcast_message(), &body)
      .await
  }

  #[napi]
  pub async fn message_content_preview(&self, message_id: String) -> Result<Buffer> {
    let result = self
      .request_get_binary(&endpoints::message_content_preview(&message_id))
      .await;
    match result {
      Ok(buffer) => Ok(buffer),
      Err(e) => Err(Error::from_reason(e.to_string())),
    }
  }

  #[napi]
  pub async fn reply_message(&self, reply_token: String, text: String) -> Result<String> {
    let body = ReplyMessageRequest {
      reply_token,
      messages: vec![TextMessage::new(text)],
    };
    self
      .request_post_json(&endpoints::reply_message(), &body)
      .await
  }

  // ── Raw JSON (accepts any LINE message types as JSON string) ──────────────────

  /// Send any message types. `messages_json` = JSON array string.
  /// e.g. '[{"type":"text","text":"Hi"},{"type":"image",...}]'
  #[napi]
  pub async fn push_messages(&self, to: String, messages_json: String) -> Result<String> {
    let messages: Vec<Value> = serde_json::from_str(&messages_json)
      .map_err(|e| Error::from_reason(format!("Invalid messages JSON: {e}")))?;
    let body = serde_json::json!({ "to": to, "messages": messages });
    self
      .request_post_json(&endpoints::push_message(), &body)
      .await
  }

  /// Reply with any message types. `messages_json` = JSON array string.
  #[napi]
  pub async fn reply_messages(&self, reply_token: String, messages_json: String) -> Result<String> {
    let messages: Vec<Value> = serde_json::from_str(&messages_json)
      .map_err(|e| Error::from_reason(format!("Invalid messages JSON: {e}")))?;
    let body = serde_json::json!({ "replyToken": reply_token, "messages": messages });
    self
      .request_post_json(&endpoints::reply_message(), &body)
      .await
  }

  /// Multicast to multiple users. `user_ids_json` = JSON string array.
  #[napi]
  pub async fn multicast(&self, user_ids_json: String, messages_json: String) -> Result<String> {
    let to: Vec<String> = serde_json::from_str(&user_ids_json)
      .map_err(|e| Error::from_reason(format!("Invalid user_ids JSON: {e}")))?;
    let messages: Vec<Value> = serde_json::from_str(&messages_json)
      .map_err(|e| Error::from_reason(format!("Invalid messages JSON: {e}")))?;
    let body = serde_json::json!({ "to": to, "messages": messages });
    self
      .request_post_json(&endpoints::multicast_message(), &body)
      .await
  }

  /// Broadcast to all followers. `messages_json` = JSON array string.
  #[napi]
  pub async fn broadcast(&self, messages_json: String) -> Result<String> {
    let messages: Vec<Value> = serde_json::from_str(&messages_json)
      .map_err(|e| Error::from_reason(format!("Invalid messages JSON: {e}")))?;
    let body = serde_json::json!({ "messages": messages });
    self
      .request_post_json(&endpoints::broadcast_message(), &body)
      .await
  }

  // ── Quota ─────────────────────────────────────────────────────────────────────

  #[napi]
  pub async fn get_message_quota(&self) -> Result<MessageQuota> {
    let body = self.request_get(&endpoints::message_quota()).await?;
    serde_json::from_str(&body).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub async fn get_message_quota_consumption(&self) -> Result<MessageQuotaConsumption> {
    let body = self
      .request_get(&endpoints::message_quota_consumption())
      .await?;
    serde_json::from_str(&body).map_err(|e| Error::from_reason(e.to_string()))
  }

  // ── Content ───────────────────────────────────────────────────────────────────

  #[napi]
  pub async fn get_message_content(&self, message_id: String) -> Result<Buffer> {
    self
      .request_get_binary(&endpoints::message_content(&message_id))
      .await
  }
}
