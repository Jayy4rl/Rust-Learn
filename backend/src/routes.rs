use axum::{
    routing::get,
    extract::Path,
    Json,
    Router,
};
use serde::Serialize;

#[derive(Serialize)]
// endpoints for access from the frontend