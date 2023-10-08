#[napi]
pub fn axum_version() -> String {
  env!("AXUM_VERSION").to_string()
}
