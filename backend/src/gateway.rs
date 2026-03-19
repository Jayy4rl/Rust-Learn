use serde::Deserialize;
use axum::{
    routing::get,
    extract::Path,
    Json,
    Router
};
use serde::Serialize;
use dotenvy::dotenv;
use std::env;
use crate::schemas::ApiListing;

// #[derive(Deserialize, Serialize)]

async fn GatewayUrl(Json(payload): Json<ApiListing>) {
    dotenv().ok();

    let apiName = payload.name;

    let slugBase: String = apiName.chars().filter(|c| c.is_ascii_lowercase() || c.is_ascii_digit()|| *c == '-' ).collect();
    let gatewayBaseUrl = env::var("GATEWAY_URL").expect("GATEWAY_URL must be set in .env"); 

    format!("{}/{}", gatewayBaseUrl, slugBase);


}

