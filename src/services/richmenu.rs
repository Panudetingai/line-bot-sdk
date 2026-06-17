use crate::api::endpoints;
use crate::client::LineClient;
use crate::types::richmenu::{RichMenu, RichMenuIdResponse, RichMenuList, RichMenuResponse};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi]
impl LineClient {
  #[napi]
  pub async fn create_rich_menu(&self, rich_menu: RichMenu) -> Result<String> {
    let body = self
      .request_post_json(&endpoints::rich_menu_create(), &rich_menu)
      .await?;
    let res: RichMenuIdResponse =
      serde_json::from_str(&body).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(res.rich_menu_id)
  }

  #[napi]
  pub async fn get_rich_menu(&self, rich_menu_id: String) -> Result<RichMenuResponse> {
    let body = self
      .request_get(&endpoints::rich_menu(&rich_menu_id))
      .await?;
    serde_json::from_str(&body).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub async fn delete_rich_menu(&self, rich_menu_id: String) -> Result<String> {
    self
      .request_delete(&endpoints::rich_menu(&rich_menu_id))
      .await
  }

  #[napi]
  pub async fn get_rich_menu_list(&self) -> Result<RichMenuList> {
    let body = self.request_get(&endpoints::rich_menu_list()).await?;
    serde_json::from_str(&body).map_err(|e| Error::from_reason(e.to_string()))
  }

  /// Upload rich menu image. `content_type` is e.g. "image/jpeg" or "image/png".
  #[napi]
  pub async fn upload_rich_menu_image(
    &self,
    rich_menu_id: String,
    image_data: Buffer,
    content_type: String,
  ) -> Result<String> {
    self
      .request_post_binary(
        &endpoints::rich_menu_image_upload(&rich_menu_id),
        image_data.to_vec(),
        &content_type,
      )
      .await
  }

  /// Download rich menu image. Returns raw bytes as Buffer.
  #[napi]
  pub async fn download_rich_menu_image(&self, rich_menu_id: String) -> Result<Buffer> {
    self
      .request_get_binary(&endpoints::rich_menu_image_download(&rich_menu_id))
      .await
  }

  // ── Default rich menu ─────────────────────────────────────────────────────────

  #[napi]
  pub async fn set_default_rich_menu(&self, rich_menu_id: String) -> Result<String> {
    let url = format!("{}/{}", endpoints::rich_menu_default(), rich_menu_id);
    self.request_post_json(&url, &serde_json::json!({})).await
  }

  #[napi]
  pub async fn cancel_default_rich_menu(&self) -> Result<String> {
    self.request_delete(&endpoints::rich_menu_default()).await
  }

  // ── User rich menu ────────────────────────────────────────────────────────────

  #[napi]
  pub async fn get_user_rich_menu(&self, user_id: String) -> Result<RichMenuIdResponse> {
    let body = self
      .request_get(&endpoints::user_rich_menu(&user_id))
      .await?;
    serde_json::from_str(&body).map_err(|e| Error::from_reason(e.to_string()))
  }

  #[napi]
  pub async fn link_rich_menu_to_user(
    &self,
    user_id: String,
    rich_menu_id: String,
  ) -> Result<String> {
    self
      .request_post_json(
        &endpoints::link_user_rich_menu(&user_id, &rich_menu_id),
        &serde_json::json!({}),
      )
      .await
  }

  #[napi]
  pub async fn unlink_rich_menu_from_user(&self, user_id: String) -> Result<String> {
    self
      .request_delete(&endpoints::user_rich_menu(&user_id))
      .await
  }

  /// Bulk link rich menu to multiple users. `user_ids_json` = JSON array string.
  #[napi]
  pub async fn bulk_link_rich_menu(
    &self,
    rich_menu_id: String,
    user_ids_json: String,
  ) -> Result<String> {
    let user_ids: Vec<String> = serde_json::from_str(&user_ids_json)
      .map_err(|e| Error::from_reason(format!("Invalid user_ids JSON: {e}")))?;
    let body = serde_json::json!({ "richMenuId": rich_menu_id, "userIds": user_ids });
    self
      .request_post_json(&endpoints::bulk_link_rich_menu(), &body)
      .await
  }

  /// Bulk unlink rich menu from multiple users. `user_ids_json` = JSON array string.
  #[napi]
  pub async fn bulk_unlink_rich_menu(&self, user_ids_json: String) -> Result<String> {
    let user_ids: Vec<String> = serde_json::from_str(&user_ids_json)
      .map_err(|e| Error::from_reason(format!("Invalid user_ids JSON: {e}")))?;
    let body = serde_json::json!({ "userIds": user_ids });
    self
      .request_post_json(&endpoints::bulk_unlink_rich_menu(), &body)
      .await
  }
}
