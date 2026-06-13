use napi::bindgen_prelude::*;
use napi_derive::napi;
use reqwest::Client;
use serde::Serialize;

#[napi]
pub struct LineClient {
  pub(crate) access_token: String,
  pub(crate) http: Client,
}

#[napi]
impl LineClient {
  #[napi(constructor)]
  pub fn new(access_token: String) -> Result<Self> {
    if access_token.trim().is_empty() {
      return Err(Error::from_reason("access_token is required"));
    }
    Ok(Self { access_token, http: Client::new() })
  }
}

impl LineClient {
  pub(crate) async fn request_get(&self, url: &str) -> Result<String> {
    let res = self
      .http
      .get(url)
      .bearer_auth(&self.access_token)
      .send()
      .await
      .map_err(|e| Error::from_reason(e.to_string()))?;

    if !res.status().is_success() {
      let status = res.status();
      let body = res.text().await.unwrap_or_default();
      return Err(Error::from_reason(format!("HTTP {status}: {body}")));
    }

    res.text().await.map_err(|e| Error::from_reason(e.to_string()))
  }

  pub(crate) async fn request_get_binary(&self, url: &str) -> Result<Buffer> {
    let res = self
      .http
      .get(url)
      .bearer_auth(&self.access_token)
      .send()
      .await
      .map_err(|e| Error::from_reason(e.to_string()))?;

    if !res.status().is_success() {
      let status = res.status();
      let body = res.text().await.unwrap_or_default();
      return Err(Error::from_reason(format!("HTTP {status}: {body}")));
    }

    let bytes = res.bytes().await.map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(Buffer::from(bytes.to_vec()))
  }

  pub(crate) async fn request_post_json<T: Serialize>(&self, url: &str, body: &T) -> Result<String> {
    let res = self
      .http
      .post(url)
      .bearer_auth(&self.access_token)
      .json(body)
      .send()
      .await
      .map_err(|e| Error::from_reason(e.to_string()))?;

    if !res.status().is_success() {
      let status = res.status();
      let text = res.text().await.unwrap_or_default();
      return Err(Error::from_reason(format!("HTTP {status}: {text}")));
    }

    let text = res.text().await.map_err(|e| Error::from_reason(e.to_string()))?;
    if text.is_empty() { Ok("{}".to_string()) } else { Ok(text) }
  }

  pub(crate) async fn request_post_binary(
    &self,
    url: &str,
    data: Vec<u8>,
    content_type: &str,
  ) -> Result<String> {
    let res = self
      .http
      .post(url)
      .bearer_auth(&self.access_token)
      .header("Content-Type", content_type)
      .body(data)
      .send()
      .await
      .map_err(|e| Error::from_reason(e.to_string()))?;

    if !res.status().is_success() {
      let status = res.status();
      let text = res.text().await.unwrap_or_default();
      return Err(Error::from_reason(format!("HTTP {status}: {text}")));
    }

    let text = res.text().await.map_err(|e| Error::from_reason(e.to_string()))?;
    if text.is_empty() { Ok("{}".to_string()) } else { Ok(text) }
  }

  pub(crate) async fn request_delete(&self, url: &str) -> Result<String> {
    let res = self
      .http
      .delete(url)
      .bearer_auth(&self.access_token)
      .send()
      .await
      .map_err(|e| Error::from_reason(e.to_string()))?;

    if !res.status().is_success() {
      let status = res.status();
      let body = res.text().await.unwrap_or_default();
      return Err(Error::from_reason(format!("HTTP {status}: {body}")));
    }

    let text = res.text().await.map_err(|e| Error::from_reason(e.to_string()))?;
    if text.is_empty() { Ok("{}".to_string()) } else { Ok(text) }
  }
}
