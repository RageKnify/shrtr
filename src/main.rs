use axum::{
    routing::{get, put, post},
    Router,
};

mod routes;
use routes::{chosen, edit, random, root, short};

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "shrtr=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    use std::time::Duration;
    use tower::ServiceBuilder;
    use tower_http::{
        auth::RequireAuthorizationLayer, compression::CompressionLayer, trace::TraceLayer,
    };

    let middleware_stack = ServiceBuilder::new()
        // timeout all requests after 5 seconds
        // .timeout(Duration::from_secs(5))
        // add high level tracing of requests and responses
        .layer(TraceLayer::new_for_http())
        // compression responses
        .layer(CompressionLayer::new());

    let app = Router::new()
        .route("/", get(root))
        .route("/r", post(random))
        .route("/c", post(chosen))
        .route("/e", put(edit))
        // add authorization for homepage, creating and modifying
        .layer(RequireAuthorizationLayer::basic("username", "password"))
        // short links don't need authorization
        .route("/s/:short", get(short))
        // add middleware to all routes
        .layer(middleware_stack);

    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    tracing::info!("will listen on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("for some reason");
}
