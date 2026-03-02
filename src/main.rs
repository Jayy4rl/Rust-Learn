use axum::{
    routing::get,
    Router,
};
use dotenvy::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();

    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set in .env");

    let pool = PgPoolOptions::new().max_connections(5).connect(&supabase_url).await?;

    println! ("Supabase connection successful");

    let row: (i64,) = sqlx::query_as("SELECT COUNT (*) FROM api_listings").fetch_one(&pool).await?;

    println!("Current row count in api_listings: {}", row.0);
    
    let app = Router::new().route("/", get(|| async {"Hello, Rust API"}));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println! ( "Server running on http://localhost:3000" );
    axum::serve(listener, app).await.unwrap();

    Ok(())

}