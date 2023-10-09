use std::{net::SocketAddr, sync::Arc};

use axum::{
  routing::{delete, get, head, options, patch, post, put, trace},
  Router,
};
use hyper::{Body, Request};
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction};
use tokio::sync::Mutex;

use super::{request::AxumRequest, response::AxumResponse};

type SharedRouter = Arc<Mutex<Router>>;

enum Method {
  Get,
  Post,
  Patch,
  Delete,
  Put,
  Options,
  Head,
  Trace,
}

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

  fn method(
    &mut self,
    method: Method,
    path: String,
    cb: ThreadsafeFunction<(AxumRequest, AxumResponse), ErrorStrategy::Fatal>,
  ) {
    let method = match method {
      Method::Get => get,
      Method::Post => post,
      Method::Patch => patch,
      Method::Delete => delete,
      Method::Put => put,
      Method::Options => options,
      Method::Head => head,
      Method::Trace => trace,
    };

    let app = Router::new().route(
      &path,
      method(|request: Request<Body>| async move {
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
  pub fn get(
    &mut self,
    path: String,
    // TODO: Probably use Either<> to have a single callback for both sync and async
    #[napi(ts_arg_type = "(request: AxumRequest, response: AxumResponse) => void")]
    cb: ThreadsafeFunction<(AxumRequest, AxumResponse), ErrorStrategy::Fatal>,
  ) {
    self.method(Method::Get, path, cb);
  }

  #[napi]
  pub fn post(
    &mut self,
    path: String,
    #[napi(ts_arg_type = "(request: AxumRequest, response: AxumResponse) => void")]
    cb: ThreadsafeFunction<(AxumRequest, AxumResponse), ErrorStrategy::Fatal>,
  ) {
    self.method(Method::Post, path, cb);
  }

  #[napi]
  pub fn patch(
    &mut self,
    path: String,
    #[napi(ts_arg_type = "(request: AxumRequest, response: AxumResponse) => void")]
    cb: ThreadsafeFunction<(AxumRequest, AxumResponse), ErrorStrategy::Fatal>,
  ) {
    self.method(Method::Patch, path, cb);
  }

  #[napi]
  pub fn delete(
    &mut self,
    path: String,
    #[napi(ts_arg_type = "(request: AxumRequest, response: AxumResponse) => void")]
    cb: ThreadsafeFunction<(AxumRequest, AxumResponse), ErrorStrategy::Fatal>,
  ) {
    self.method(Method::Delete, path, cb);
  }

  #[napi]
  pub fn put(
    &mut self,
    path: String,
    #[napi(ts_arg_type = "(request: AxumRequest, response: AxumResponse) => void")]
    cb: ThreadsafeFunction<(AxumRequest, AxumResponse), ErrorStrategy::Fatal>,
  ) {
    self.method(Method::Put, path, cb);
  }

  #[napi]
  pub fn options(
    &mut self,
    path: String,
    #[napi(ts_arg_type = "(request: AxumRequest, response: AxumResponse) => void")]
    cb: ThreadsafeFunction<(AxumRequest, AxumResponse), ErrorStrategy::Fatal>,
  ) {
    self.method(Method::Options, path, cb);
  }

  #[napi]
  pub fn head(
    &mut self,
    path: String,
    #[napi(ts_arg_type = "(request: AxumRequest, response: AxumResponse) => void")]
    cb: ThreadsafeFunction<(AxumRequest, AxumResponse), ErrorStrategy::Fatal>,
  ) {
    self.method(Method::Head, path, cb);
  }

  #[napi]
  pub fn trace(
    &mut self,
    path: String,
    #[napi(ts_arg_type = "(request: AxumRequest, response: AxumResponse) => void")]
    cb: ThreadsafeFunction<(AxumRequest, AxumResponse), ErrorStrategy::Fatal>,
  ) {
    self.method(Method::Trace, path, cb);
  }

  #[napi]
  pub async fn listen(&self, port: u16, cb: ThreadsafeFunction<(), ErrorStrategy::Fatal>) {
    let mut app = Router::new();
    for route in self.routes.iter() {
      app = app.nest("/", route.lock().await.clone());
    }
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    // Start axum in a background task, so we can await on it, and then call the callback
    // when the task is created. And then block the main thread the task end.
    let task = tokio::task::spawn(async move {
      axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    });

    // What values should we inject into this callback?
    // I was thinking in something like starting a benchmark here and injecting a start in x ms
    // I currently don't know how what to pass
    cb.call_async::<()>(()).await.unwrap();

    task.await.unwrap();
  }
}

impl Default for AxumApp {
  fn default() -> Self {
    Self::new()
  }
}
