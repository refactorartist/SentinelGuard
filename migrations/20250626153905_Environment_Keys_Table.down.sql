-- Add down migration script here
DROP INDEX IF EXISTS idx_environment_key_environment_id_algorithm;
DROP TABLE IF EXISTS environment_key;