-- Add up migration script here
CREATE TABLE IF NOT EXISTS service_account (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    email TEXT NOT NULL, 
    secret TEXT NOT NULL,
    description TEXT NOT NULL,
    enabled BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_service_account_name ON service_account(name);
CREATE INDEX IF NOT EXISTS idx_service_account_enabled ON service_account(enabled);

