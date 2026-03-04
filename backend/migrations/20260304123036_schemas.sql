-- Add migration script here
-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- API Listings table
CREATE TABLE IF NOT EXISTS api_listings (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name VARCHAR(255) NOT NULL,
  description TEXT,
  base_url TEXT NOT NULL,
  api_key TEXT,
  price_per_call VARCHAR(50) NOT NULL,
  category VARCHAR(100) NOT NULL,
  status VARCHAR(50) NOT NULL DEFAULT 'active',
  source VARCHAR(50) NOT NULL,
  owner TEXT NOT NULL,
  total_calls BIGINT DEFAULT 0,
  revenue VARCHAR(50) DEFAULT '0',
  tags TEXT[],
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- API Endpoints table
CREATE TABLE IF NOT EXISTS api_endpoints (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  api_id UUID REFERENCES api_listings(id) ON DELETE CASCADE,
  path VARCHAR(500) NOT NULL,
  method VARCHAR(10) NOT NULL,
  summary TEXT,
  description TEXT,
  parameters JSONB,
  request_body JSONB,
  responses JSONB,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- API Usage table
CREATE TABLE IF NOT EXISTS api_usage (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  api_id UUID REFERENCES api_listings(id) ON DELETE CASCADE,
  user_address TEXT NOT NULL,
  timestamp TIMESTAMPTZ DEFAULT NOW(),
  success BOOLEAN NOT NULL,
  error TEXT,
  cost VARCHAR(50) NOT NULL
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_api_listings_owner ON api_listings(owner);
CREATE INDEX IF NOT EXISTS idx_api_listings_status ON api_listings(status);
CREATE INDEX IF NOT EXISTS idx_api_endpoints_api_id ON api_endpoints(api_id);
CREATE INDEX IF NOT EXISTS idx_api_endpoints_method ON api_endpoints(method);
CREATE INDEX IF NOT EXISTS idx_api_usage_api_id ON api_usage(api_id);
CREATE INDEX IF NOT EXISTS idx_api_usage_user_address ON api_usage(user_address);
