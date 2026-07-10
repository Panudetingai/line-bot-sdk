use crate::api::endpoints;
use crate::client::LineClient;
use crate::types::insight::{FollowersInsight, MessageDeliveryOverview};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
impl LineClient {
  /// `date` format: YYYYMMDD (e.g. "20240101")
  #[napi]
  pub async fn get_message_delivery_overview(
    &self,
    date: String,
  ) -> Result<MessageDeliveryOverview> {
    let body = self
      .request_get(&endpoints::insight_message_delivery(&date))
      .await?;
    serde_json::from_str(&body).map_err(|e| Error::from_reason(e.to_string()))
  }

  /// `date` format: YYYYMMDD
  #[napi]
  pub async fn get_followers_insight(&self, date: String) -> Result<FollowersInsight> {
    let body = self
      .request_get(&endpoints::insight_followers(&date))
      .await?;
    serde_json::from_str(&body).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub async fn get_demographic(&self) -> Result<String> {
    let body = self.request_get(&endpoints::insight_demographic()).await?;
    serde_json::from_str(&body).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub async fn get_webhook_endpoint(&self) -> Result<String> {
    let body = self.request_get(&endpoints::webhook_endpoint()).await?;
    Ok(body)
  }
}
