use base64::{engine::general_purpose::STANDARD, Engine};
use hmac::{Hmac, Mac};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use sha2::Sha256;

use crate::types::webhook::WebhookBody;

/// Verify the X-Line-Signature header against the raw request body.
/// Returns true if the signature is valid.
#[napi]
pub fn verify_webhook_signature(body: String, signature: String, channel_secret: String) -> bool {
  let Ok(sig_bytes) = STANDARD.decode(&signature) else {
    return false;
  };
  let mut mac =
    Hmac::<Sha256>::new_from_slice(channel_secret.as_bytes()).expect("HMAC accepts any key size");
  mac.update(body.as_bytes());
  mac.verify_slice(&sig_bytes).is_ok()
}

/// Parse a webhook body JSON string into typed events.
/// Throws if the JSON is invalid.
#[napi]
pub fn parse_webhook_body(body: String) -> Result<WebhookBody> {
  serde_json::from_str(&body).map_err(|e| Error::from_reason(format!("Invalid webhook body: {e}")))
}
