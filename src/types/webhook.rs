use napi_derive::napi;
use serde::{Deserialize, Serialize};

// ------- Verify Signature ──────────────────────────────────────────────────────
#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifySignatureArgs {
  pub body: String,
  pub signature: String,
  pub channel_secret: String,
}

// ── Webhook Body ──────────────────────────────────────────────────────────────

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookBody {
  pub destination: String,
  pub events: Vec<WebhookEvent>,
}

// ── Event ────────────────────────────────────────────────────────────────────

// EVENT TYPES
#[napi(string_enum = "camelCase")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EventType {
  /// ผู้ใช้งานส่งข้อความ รูปภาพ วิดีโอ ไฟล์ หรือสติกเกอร์เข้ามาในแชท
  Message,

  /// หมายเหตุ: LINE Webhook ไม่มีเหตุการณ์ 'edit' ข้อความ
  /// แต่จะใช้สำหรับระบุเหตุการณ์อื่นๆ ตามสถาปัตยกรรมระบบของคุณเอง
  Edit,

  /// ผู้ใช้งานกดปุ่มที่มี Action เป็น Postback (เช่น ปุ่มใน Flex Message หรือ Quick Reply)
  Postback,

  /// ผู้ใช้งานเพิ่มบัญชีทางการเป็นเพื่อน หรือเปลี่ยนสถานะจากบล็อกเป็นเลิกบล็อก
  Follow,

  /// ผู้ใช้งานบล็อกบัญชีทางการ หรือลบระบบออกจากรายชื่อเพื่อน
  Unfollow,

  /// บัญชีทางการ (Bot) ถูกเชิญหรือเพิ่มเข้าไปในกลุ่ม (Group) หรือห้องแชท (Room)
  Join,

  /// บัญชีทางการ (Bot) ถูกลบหรือเตะออกจากกลุ่ม (Group) หรือห้องแชท (Room)
  Leave,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookEvent {
  #[serde(rename = "type")]
  pub event_type: EventType,
  pub timestamp: i64,
  pub source: EventSource,
  pub webhook_event_id: Option<String>,
  pub delivery_context: Option<DeliveryContext>,

  // message event
  pub reply_token: Option<String>,
  pub message: Option<EventMessage>,

  // postback event
  pub postback: Option<PostbackContent>,

  // follow/unfollow event
  // (no extra fields)

  // join/leave event
  // (no extra fields)

  // beacon event
  pub beacon: Option<BeaconContent>,

  // account link event
  pub link: Option<LinkContent>,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventSource {
  #[serde(rename = "type")]
  pub source_type: String,
  pub user_id: Option<String>,
  pub group_id: Option<String>,
  pub room_id: Option<String>,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryContext {
  pub is_redelivery: bool,
}

// ── Event Message ─────────────────────────────────────────────────────────────

// MESSAGE TYPES
#[napi(string_enum = "camelCase")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MessageType {
  Text,
  Image,
  Video,
  Audio,
  File,
  Location,
  Sticker,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventMessage {
  pub id: String,
  #[serde(rename = "type")]
  pub message_type: MessageType,

  // text message
  pub text: Option<String>,

  // image/video/audio/file message
  pub content_provider: Option<ContentProvider>,
  pub duration: Option<i64>,

  // file message
  pub file_name: Option<String>,
  pub file_size: Option<i64>,

  // location message
  pub title: Option<String>,
  pub address: Option<String>,
  pub latitude: Option<f64>,
  pub longitude: Option<f64>,

  // sticker message
  pub package_id: Option<String>,
  pub sticker_id: Option<String>,
  pub sticker_resource_type: Option<String>,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentProvider {
  #[serde(rename = "type")]
  pub provider_type: String,
  pub original_content_url: Option<String>,
  pub preview_image_url: Option<String>,
}

// ── Postback ──────────────────────────────────────────────────────────────────

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostbackContent {
  pub data: String,
  pub params: Option<PostbackParams>,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostbackParams {
  pub date: Option<String>,
  pub time: Option<String>,
  pub datetime: Option<String>,
}

// ── Beacon ───────────────────────────────────────────────────────────────────

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BeaconContent {
  pub hwid: String,
  #[serde(rename = "type")]
  pub beacon_type: String,
  pub dm: Option<String>,
}

// ── Account Link ──────────────────────────────────────────────────────────────

#[napi(object)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkContent {
  pub result: String,
  pub nonce: String,
}
