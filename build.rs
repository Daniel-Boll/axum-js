extern crate napi_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  napi_build::setup();

  // ------ CONFIGURE AXUM VERSION ------

  let cargo_toml_path = std::path::Path::new("Cargo.toml");

  let mut cargo_toml = String::new();
  use std::io::Read;
  std::fs::File::open(cargo_toml_path)?.read_to_string(&mut cargo_toml)?;

  // Define the structure to hold the part of Cargo.toml that we are interested in
  #[derive(serde::Deserialize)]
  struct Config {
    dependencies: Dependencies,
  }

  #[derive(serde::Deserialize)]
  struct Dependencies {
    axum: ExpandedConfig,
  }

  #[derive(serde::Deserialize)]
  struct ExpandedConfig {
    version: String,
  }

  let config: Config = toml::from_str(&cargo_toml)?;

  println!(
    "cargo:rustc-env=AXUM_VERSION={}",
    config.dependencies.axum.version
  );

  Ok(())
}
