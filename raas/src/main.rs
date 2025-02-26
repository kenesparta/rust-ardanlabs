use axum::extract::{Path, State};
use axum::extract::{Query, Request};
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::Next;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{middleware, Router};
use axum::{Extension, Json};
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tower_http::compression::CompressionLayer;

struct Counter {
    counter: AtomicUsize,
}

struct Config {
    config_string: String,
}

struct MySate(i32);

fn hello_service() -> Router {
    let state = Arc::new(MySate(5));
    Router::new().route("/", get(sv1_handler)).with_state(state)
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
        .nest("/hello", hello_service())
        .route("/", get(handler))
        .route("/status", get(status))
        .route("/book/{id}", get(path_extract))
        .route("/book", get(query_extractor))
        .route("/header", get(header_extractor))
        .fallback_service(ServeDir::new("web"))
        .layer(Extension(shared_counter))
        .layer(Extension(shared_text))
        .layer(CompressionLayer::new())
        .route_layer(middleware::from_fn(auth));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3003")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn sv1_handler(
    Extension(counter): Extension<Arc<Counter>>,
    State(state): State<Arc<MySate>>,
) -> Html<String> {
    counter
        .counter
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Html(format!(
        "<pre>My State: {} & You are the visitor number: {}</pre>",
        state.0,
        counter.counter.load(std::sync::atomic::Ordering::Relaxed),
    ))
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

async fn status() -> Result<impl IntoResponse, (StatusCode, String)> {
    // StatusCode::IM_A_TEAPOT
    let start = std::time::SystemTime::now();

    let second_wrapped = start
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Bad Clock".to_string()))?
        .as_secs()
        % 3;

    let divided = 100u64
        .checked_div(second_wrapped)
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "div by 0".to_string()))?;
    Ok(Json(divided))
}

#[derive(Clone)]
struct AuthHeader {
    id: String,
}

// Middleware Auth With Injection
// Selectively applying layers
// Router Layers
async fn auth(
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Some(header) = headers.get("x-request-id") {
        let header = header.to_str().unwrap();
        if header == "1234" {
            req.extensions_mut().insert(AuthHeader {
                id: header.to_string(),
            });
            return Ok(next.run(req).await);
        }
    }
    Err((StatusCode::UNAUTHORIZED, "Header not valid".to_string()))
}
