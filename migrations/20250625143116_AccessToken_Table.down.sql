-- Add down migration script here
DROP INDEX IF EXISTS idx_access_tokens_project_access_id;


DROP TABLE IF EXISTS access_tokens;