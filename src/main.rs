use axum::{
    Router,
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use std::net::SocketAddr;
use std::time::Instant;

async fn log_middleware(req: Request, next: Next) -> Response {
    let start = Instant::now();
    let path = req.uri().path().to_string();
    println!("GET {}", path);
    let response = next.run(req).await;
    let duration = start.elapsed();

    println!("The answer was done for '{}' in {:?}", path, duration);
    response
}

async fn hello() -> &'static str {
    "hello from axum"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .layer(middleware::from_fn(log_middleware));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("server running on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
