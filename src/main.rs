use axum::{routing::get, Router, response::Html};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<String> {
    let html = tokio::fs::read_to_string("public/index.html").await.unwrap();
    axum::response::Html(html)
}


