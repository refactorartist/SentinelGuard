-- Add down migration script here
DROP INDEX IF EXISTS idx_service_account_name;
DROP INDEX IF EXISTS idx_service_account_email;
DROP INDEX IF EXISTS idx_service_account_enabled;

DROP TABLE IF EXISTS service_account;