use axum::{http::Uri, Json, extract::Path, response::Redirect};

use serde::Deserialize;

pub(crate) async fn root() -> &'static str {
    "Hello, World! Soon this will be a usable UI"
}

#[derive(Deserialize, Debug)]
pub(crate) struct NewURL {
    long: String,
}

pub(crate) async fn random(Json(new): Json<NewURL>) {
    tracing::info!("random {}", new.long);
    let short: u64 = todo!("generate new random u64");
    match Uri::try_from(new.long) {
        Ok(_) => todo!("happy path"),
        Err(err) => todo!("sad path"),
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct NewChosenURL {
    short: String,
    long: String,
}

pub(crate) async fn chosen(Json(new): Json<NewChosenURL>) {
    tracing::info!("chosen {}: {}", new.short, new.long);
    todo!("check that new.short isn't in use");
    match Uri::try_from(new.long) {
        Ok(_) => todo!("happy path"),
        Err(err) => todo!("sad path"),
    }
}

pub(crate) async fn short(Path(key): Path<String>) -> Redirect {
    tracing::info!("short {}", key);
    let uri = Uri::builder()
        .scheme("https")
        .authority("jplborges.pt")
        .path_and_query("")
        .build().expect("this should URI build without problems");
    Redirect::temporary(uri)
}
