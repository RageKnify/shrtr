use axum::{
    extract::Path,
    response::Redirect,
    routing::{get, post},
    Json, Router,
};

use axum::http::Uri;


use serde::Deserialize;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "shrtr=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    use tower_http::{auth::AddAuthorizationLayer, compression::CompressionLayer, trace::TraceLayer};
    use tower::ServiceBuilder;
    use std::time::Duration;

    let middleware_stack = ServiceBuilder::new()
        // timeout all requests after 5 seconds
        // .timeout(Duration::from_secs(5))
        // add high level tracing of requests and responses
        .layer(TraceLayer::new_for_http())
        // add autorization
        // .layer(AddAuthorizationLayer::basic("username", "password"))
        // compression responses
        .layer(CompressionLayer::new());

    let app = Router::new().route("/", get(root)).route("/r", post(random)).route("/c", post(chosen)).route("/s/:short", get(short)).layer(middleware_stack);

    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    tracing::info!("will listen on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("for some reason");
}

async fn root() -> &'static str {
    "Hello, World! Soon this will be a usable UI"
}

#[derive(Deserialize, Debug)]
struct NewURL {
    long: String,
}

async fn random(Json(new): Json<NewURL>) {
    tracing::info!("random {}", new.long);
    let short: u64 = todo!("generate new random u64");
    match Uri::try_from(new.long) {
        Ok(_) => todo!("happy path"),
        Err(err) => todo!("sad path"),
    }
}

#[derive(Deserialize, Debug)]
struct NewChosenURL {
    short: String,
    long: String,
}

async fn chosen(Json(new): Json<NewChosenURL>) {
    tracing::info!("chosen {}: {}", new.short, new.long);
    todo!("check that new.short isn't in use");
    match Uri::try_from(new.long) {
        Ok(_) => todo!("happy path"),
        Err(err) => todo!("sad path"),
    }
}

async fn short(Path(key): Path<String>) -> Redirect {
    tracing::info!("short {}", key);
    let uri = Uri::builder()
        .scheme("https")
        .authority("jplborges.pt")
        .path_and_query("")
        .build().expect("this should URI build without problems");
    Redirect::temporary(uri)
}
