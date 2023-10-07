use std::{net::SocketAddr, sync::Arc};

use axum::{routing::get, Router};
use tokio::sync::Mutex;

type SharedRouter = Arc<Mutex<Router>>;

#[napi]
pub struct AxumApp {
  routes: Vec<SharedRouter>,
}

#[napi]
impl AxumApp {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self { routes: vec![] }
  }

  #[napi]
  pub fn get(&mut self, path: String) {
    let app = Router::new().route(&path, get(|| async { "Hello World!" }));

    self.routes.push(Arc::new(Mutex::new(app)));
  }

  #[napi]
  pub async fn listen(&self, port: u16) {
    let mut app = Router::new();
    for route in self.routes.iter() {
      app = app.nest("/", route.lock().await.clone());
    }
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    axum::Server::bind(&addr)
      .serve(app.into_make_service())
      .await
      .unwrap();
  }
}

impl Default for AxumApp {
  fn default() -> Self {
    Self::new()
  }
}
