#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn axum_version() -> String {
  env!("AXUM_VERSION").to_string()
}
