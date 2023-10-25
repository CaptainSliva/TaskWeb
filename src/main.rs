use askama::Template;
use axum::routing::get;
use dotenv::dotenv;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

async fn index() -> IndexTemplate {
    IndexTemplate
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = axum::Router::new()
        .route("/", get(index));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("http://127.0.0.1:8080/");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Bind server fail");
}