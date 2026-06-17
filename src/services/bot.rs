use crate::api::endpoints;
use crate::client::LineClient;
use crate::types::profile::BotInfo;
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
impl LineClient {
  #[napi]
  pub async fn verify(&self) -> Result<BotInfo> {
    let body = self.request_get(&endpoints::bot_info()).await?;
    serde_json::from_str(&body).map_err(|e| Error::from_reason(e.to_string()))
  }
}
