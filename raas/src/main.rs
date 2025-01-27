use axum::extract::Path;
use axum::extract::Query;
use axum::http::HeaderMap;
use axum::response::Html;
use axum::routing::get;
use axum::Extension;
use axum::Router;
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

struct Counter {
    counter: AtomicUsize,
}

struct Config {
    config_string: String,
}

#[tokio::main]
async fn main() {
    let shared_counter = Arc::new(Counter {
        counter: AtomicUsize::new(0),
    });

    let shared_text = Arc::new(Config {
        config_string: "This is my configuration".to_string(),
    });

    let app = Router::new()
        .route("/", get(handler))
        .route("/book/{id}", get(path_extract))
        .route("/book", get(query_extractor))
        .route("/header", get(header_extractor))
        .layer(Extension(shared_counter))
        .layer(Extension(shared_text));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3003")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(
    Extension(counter): Extension<Arc<Counter>>,
    Extension(config): Extension<Arc<Config>>,
) -> Html<String> {
    counter
        .counter
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Html(format!(
        "<pre>{} You are the visitor number: {}</pre>",
        config.config_string,
        counter.counter.load(std::sync::atomic::Ordering::Relaxed),
    ))
}

async fn path_extract(Path(id): Path<u32>) -> Html<String> {
    Html(format!("<pre>Book ID: {}</pre>", id))
}

async fn query_extractor(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    Html(format!("<pre>Query Parameters: {params:#?}</pre>"))
}

async fn header_extractor(headers: HeaderMap) -> Html<String> {
    Html(format!("<pre>Query Parameters: {headers:#?}</pre>"))
}
