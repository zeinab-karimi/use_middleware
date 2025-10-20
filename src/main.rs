use axum::{
    Router,
    http::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
    body::Body,
};
use std::net::SocketAddr;
use std::time::Instant;

async fn log_requests(req:Request<Body>, next:Next) -> Response {
    //middleware for logging
    let start = Instant::now();
    let path = req.uri().path().to_string();
    println!("GET {}", path);

    //moving from the middleware to the next handler
    let response = next.run(req).await;
    let duration = start.elapsed();

    println!("The answer was done for '{}' in {:?}", path, duration);
    response
}

//routes
async fn home() -> &'static str {
    "welcome to the main page"
}

async fn list_users() -> &'static str {
    "list of users"
}
#[tokio::main]
async fn main() {
    //Definition Router specific to users(with middleware)
    let users_router = Router::new()
        .route("/users", get(list_users))
        .layer(middleware::from_fn(log_requests)); //it is only active for this route

    //define the main route
    let app = Router::new().route("/", get(home)).merge(users_router); //we combine two router

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("server running on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
