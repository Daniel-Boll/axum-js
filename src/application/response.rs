use std::{
  str::FromStr,
  sync::{Arc, Mutex},
};

use axum::{
  http::{HeaderName, HeaderValue},
  response::IntoResponse,
};
use hyper::StatusCode;
use napi::bindgen_prelude::Result;

#[napi]
pub struct AxumResponseInternal {
  status: Option<StatusCode>,
  headers: Option<hyper::HeaderMap>,
  body: Option<serde_json::Value>,
}

#[napi]
#[derive(Clone)]
pub struct AxumResponse {
  inner: Arc<Mutex<AxumResponseInternal>>,
}

#[napi]
impl AxumResponseInternal {
  pub fn new() -> Self {
    Self {
      status: None,
      headers: None,
      body: None,
    }
  }

  pub fn send_json(&mut self, body: serde_json::Value) {
    self
      .set_header("Content-Type".to_string(), "application/json".to_string())
      .unwrap();

    self.body = Some(body);
  }

  pub fn status(&mut self, status: u16) {
    self.status = Some(StatusCode::from_u16(status).unwrap());
  }

  pub fn set_header(&mut self, key: String, value: String) -> Result<()> {
    let mut headers = match &self.headers {
      Some(headers) => headers.clone(),
      None => hyper::HeaderMap::new(),
    };

    let (key, value) = match (HeaderName::from_str(&key), HeaderValue::from_str(&value)) {
      (Ok(key), Ok(value)) => (key, value),
      (Ok(_), Err(_)) => {
        return Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("Invalid header value: {}", value),
        ));
      }
      (Err(_), Ok(_)) => {
        return Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("Invalid header key: {}", key),
        ));
      }
      (Err(_), Err(_)) => {
        return Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!("Invalid header key: {}, value: {}", key, value),
        ));
      }
    };

    headers.insert(key, value);

    self.headers = Some(headers);

    Ok(())
  }

  pub fn aggregate(&self) -> impl IntoResponse {
    (
      self.status.unwrap_or(StatusCode::OK),
      self.headers.clone().unwrap_or_default(),
      serde_json::to_string(&self.body).unwrap_or_default(),
    )
  }

  pub fn produce_response(&self) -> axum::response::Response {
    self.aggregate().into_response()
  }
}

impl Default for AxumResponseInternal {
  fn default() -> Self {
    Self::new()
  }
}

impl IntoResponse for AxumResponseInternal {
  fn into_response(self) -> axum::response::Response {
    self.aggregate().into_response()
  }
}

#[napi]
impl AxumResponse {
  pub fn new() -> Self {
    Self {
      inner: Arc::new(Mutex::new(AxumResponseInternal::new())),
    }
  }

  #[napi]
  pub fn send_json(&mut self, body: serde_json::Value) -> &Self {
    self.inner.lock().unwrap().send_json(body);
    self
  }

  #[napi]
  pub fn status(&mut self, status: u16) -> &Self {
    self.inner.lock().unwrap().status(status);
    self
  }

  #[napi]
  pub fn set_header(&mut self, key: String, value: String) -> &Self {
    self.inner.lock().unwrap().set_header(key, value).unwrap();
    self
  }

  pub fn aggregate(&self) -> impl IntoResponse {
    self.inner.lock().unwrap().aggregate()
  }

  pub fn produce_response(&self) -> axum::response::Response {
    self.inner.lock().unwrap().produce_response()
  }
}

impl Default for AxumResponse {
  fn default() -> Self {
    Self::new()
  }
}
