use std::collections::HashMap;
use axum::extract::Path;
use axum::extract::Query;
use axum::response::Html;
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/book/{id}", get(path_extract))
        .route("/book", get(query_extractor));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3003")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<pre>OK!</pre>")
}

async fn path_extract(Path(id): Path<u32>) -> Html<String> {
    Html(format!("<pre>Book ID: {}</pre>", id))
}

async fn query_extractor(Query(params): Query<HashMap<String,String>>) -> Html<String> {
    Html(format!("<pre>Query Parameters: {params:#?}</pre>"))
}