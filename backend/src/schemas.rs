use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::Value;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]

pub struct ApiListing{
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub base_url: String,
    pub api_key: Option<String>,
    pub price_per_call: String,
    pub category: String,
    pub status: String,
    pub source: String,
    pub owner: String,
    pub total_calls: i32,
    pub revenue: String,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Vec<String>,
}

pub struct APIEndpoint{
    pub id: Uuid,
    pub api_id: String,
    pub path: String,
    pub method: String,
    pub summary: String,
    pub description: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub request_body: HashMap<String, serde_json::Value>,
    pub responses: HashMap<String, serde_json::Value>,
    pub created_at: String,
}

pub struct APIUsage{
    pub id: Uuid,
    pub api_id: String,
    pub user_address: String,
    pub timestamp: String,
    pub success: bool,
    pub error: String,
    pub cost: String,
}