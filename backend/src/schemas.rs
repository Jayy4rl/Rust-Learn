use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]

pub struct ApiListing{
    pub id: Uuid,
    pub name: string,
    pub description: string,
    pub base_url: string,
    pub api_key?: string | null;
    pub price_per_call: string;
    pub category: string;
    pub status: string;
    pub source: string;
    pub owner: string;
    pub total_calls: number;
    pub revenue: string;
    pub created_at: string;
    pub updated_at: string;
    pub tags?: string[];
}

pub struct APIEndpoint{
    pub id: Uuid;
    pub api_id: string;
    pub path: string;
    pub method: string;
    pub summary?: string;
    pub description?: string;
    pub parameters?: Record<string, unknown>;
    pub request_body?: Record<string, unknown>;
    pub responses?: Record<string, unknown>;
    pub created_at: string;
}

pub struct APIUsage{
    pub id: Uuid;
    pub api_id: string;
    pub user_address: string;
    pub timestamp: string;
    pub success: boolean;
    pub error?: string;
    pub cost: string;
}