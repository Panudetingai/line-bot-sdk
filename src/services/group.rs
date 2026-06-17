use crate::api::endpoints;
use crate::client::LineClient;
use crate::types::group::{GroupMemberProfile, GroupSummary, MemberIds};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
impl LineClient {
  // ── Group ─────────────────────────────────────────────────────────────────────

  #[napi]
  pub async fn get_group_summary(&self, group_id: String) -> Result<GroupSummary> {
    let body = self
      .request_get(&endpoints::group_summary(&group_id))
      .await?;
    serde_json::from_str(&body).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub async fn get_group_members_count(&self, group_id: String) -> Result<i64> {
    let body = self.request_get(&endpoints::group_members_count(&group_id)).await?;
    match serde_json::from_str(&body).map_err(|e| Error::from_reason(e.to_string())) {
        Ok(count) => Ok(count),
        Err(_) => Err(Error::from_reason("Failed to parse group members count")),
    }
  }

  #[napi]
  pub async fn get_group_member_profile(
    &self,
    group_id: String,
    user_id: String,
  ) -> Result<GroupMemberProfile> {
    let body = self
      .request_get(&endpoints::group_member_profile(&group_id, &user_id))
      .await?;
    serde_json::from_str(&body).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub async fn get_group_member_ids(
    &self,
    group_id: String,
    next: Option<String>,
  ) -> Result<MemberIds> {
    let base = endpoints::group_member_ids(&group_id);
    let url = match next.as_deref() {
      Some(c) if !c.is_empty() => format!("{base}?start={c}"),
      _ => base,
    };
    let body = self.request_get(&url).await?;
    serde_json::from_str(&body).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub async fn leave_group(&self, group_id: String) -> Result<String> {
    self
      .request_post_json(&endpoints::leave_group(&group_id), &serde_json::json!({}))
      .await
  }

  // ── Room ──────────────────────────────────────────────────────────────────────

  #[napi]
  pub async fn get_room_member_profile(
    &self,
    room_id: String,
    user_id: String,
  ) -> Result<GroupMemberProfile> {
    let body = self
      .request_get(&endpoints::room_member_profile(&room_id, &user_id))
      .await?;
    serde_json::from_str(&body).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub async fn get_room_member_ids(
    &self,
    room_id: String,
    next: Option<String>,
  ) -> Result<MemberIds> {
    let base = endpoints::room_member_ids(&room_id);
    let url = match next.as_deref() {
      Some(c) if !c.is_empty() => format!("{base}?start={c}"),
      _ => base,
    };
    let body = self.request_get(&url).await?;
    serde_json::from_str(&body).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub async fn leave_room(&self, room_id: String) -> Result<String> {
    self
      .request_post_json(&endpoints::leave_room(&room_id), &serde_json::json!({}))
      .await
  }

}
