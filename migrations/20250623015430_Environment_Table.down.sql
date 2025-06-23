-- Add down migration script here
DROP INDEX idx_environment_project_id_name;
DROP TABLE environment;
