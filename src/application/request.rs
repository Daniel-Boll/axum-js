type AxumRequestBody = axum::http::Request<axum::body::Body>;

#[napi]
pub struct AxumRequest {
  body: Option<hyper::body::Bytes>,
}

#[napi]
impl AxumRequest {
  pub async fn new(request: AxumRequestBody) -> Self {
    let mut request = request;

    let body_bytes = match hyper::body::to_bytes(request.body_mut()).await {
      Ok(b) => Some(b),
      Err(_) => None,
    };

    Self { body: body_bytes }
  }

  #[napi(ts_return_type = "Record<string, any>")]
  #[napi(getter)]
  pub fn body(&mut self) -> serde_json::Value {
    match &self.body {
      Some(bytes) => match serde_json::from_slice(bytes.as_ref()) {
        Ok(value) => value,
        Err(_) => serde_json::Value::Null,
      },
      None => serde_json::Value::Null,
    }
  }
}
