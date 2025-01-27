use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

struct Config {
    config_string: String,
    counter: AtomicUsize,
}

#[tokio::main]
async fn main() {
    let shared_config = Arc::new(Config {
        config_string: "My Config string".to_string(),
        counter: AtomicUsize::new(0),
    });

    let app = Router::new()
        .route("/", get(handler))
        .route("/book/{id}", get(path_extract))
        .route("/book", get(query_extractor))
        .route("/header", get(header_extractor))
        .with_state(shared_config);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3003")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(State(config): State<Arc<Config>>) -> Html<String> {
    config
        .counter
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Html(format!(
        "<h1>You are the visitor number: {}, config: {}</h1>",
        config.counter.load(std::sync::atomic::Ordering::Relaxed),
        config.config_string
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
