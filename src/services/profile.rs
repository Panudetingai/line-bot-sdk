use crate::api::endpoints;
use crate::client::LineClient;
use crate::types::group::FollowerIds;
use crate::types::profile::Profile;
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
impl LineClient {
  #[napi]
  pub async fn get_profile(&self, user_id: String) -> Result<Profile> {
    let body = self.request_get(&endpoints::profile(&user_id)).await?;
    serde_json::from_str(&body).map_err(|e| Error::from_reason(e.to_string()))
  }

  /// Returns follower user IDs. Pass `next` cursor for pagination (empty string for first page).
  #[napi]
  pub async fn get_follower_ids(&self, next: Option<String>) -> Result<FollowerIds> {
    let url = match next.as_deref() {
      Some(c) if !c.is_empty() => format!("{}?start={c}", endpoints::followers_ids()),
      _ => endpoints::followers_ids(),
    };
    let body = self.request_get(&url).await?;
    serde_json::from_str(&body).map_err(|e| Error::from_reason(e.to_string()))
  }
}
