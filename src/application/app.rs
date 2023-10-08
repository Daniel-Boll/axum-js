use std::{net::SocketAddr, sync::Arc};

use axum::{routing::get, Router};
use hyper::{Body, Request};
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use tokio::sync::Mutex;

use super::{request::AxumRequest, response::AxumResponse};

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
  pub fn get(
    &mut self,
    path: String,
    // TODO: Probably use Either<> to have a single callback for both sync and async
    #[napi(ts_arg_type = "(request: AxumRequest, response: AxumResponse) => void")]
    cb: ThreadsafeFunction<(AxumRequest, AxumResponse), ErrorStrategy::Fatal>,
  ) {
    let app = Router::new().route(
      &path,
      get(|request: Request<Body>| async move {
        let request = AxumRequest::new(request).await;
        let response = AxumResponse::new();

        cb.call_async::<()>((request, response.clone()))
          .await
          .unwrap();

        response.produce_response()
      }),
    );

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
