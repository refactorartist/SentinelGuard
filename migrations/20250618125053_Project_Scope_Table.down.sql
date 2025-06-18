-- Add down migration script here
DROP INDEX IF EXISTS idx_project_scopes_project_id_scope;
DROP INDEX IF EXISTS idx_project_scopes_project_id;

DROP TABLE IF EXISTS project_scopes;
