#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use axum::response::Html;

pub async fn echo(req_body: String) -> String {
    req_body
}

pub async fn hello(State(_ctx): State<AppContext>) ->  Result<Html<String>> {
    // do something with context (database, etc)
    format::html("<h1>hello</h1>")
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(hello))
        .add("/echo", post(echo))
}
