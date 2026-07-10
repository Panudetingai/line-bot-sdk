use base64::{engine::general_purpose::STANDARD, Engine};
use hmac::{Hmac, Mac};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use sha2::Sha256;

use crate::types::webhook::{VerifySignatureArgs, WebhookBody};

/// Verify the X-Line-Signature header against the raw request body.
/// Returns true if the signature is valid.

#[napi]
pub fn verify_webhook_signature(
    args: VerifySignatureArgs,
) -> napi::Result<bool> {
    let sig_bytes = STANDARD
        .decode(&args.signature)
        .map_err(|e| napi::Error::from_reason(format!("Invalid signature: {e}")))?;

    let mut mac = Hmac::<Sha256>::new_from_slice(args.channel_secret.as_bytes())
        .map_err(|e| napi::Error::from_reason(format!("Invalid channel secret: {e}")))?;

    mac.update(args.body.as_bytes());

    match mac.verify_slice(&sig_bytes) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false), // Signature ไม่ตรง ถือว่าไม่ใช่ error
    }
}

/// Parse a webhook body JSON string into typed events.
/// Throws if the JSON is invalid.
#[napi]
pub fn parse_webhook_body(body: String) -> Result<WebhookBody> {
  serde_json::from_str(&body).map_err(|e| Error::from_reason(format!("Invalid webhook body: {e}")))
}
