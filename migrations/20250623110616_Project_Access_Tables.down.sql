-- Add down migration script here
DROP INDEX idx_project_access_scopes_project_access_id_scope_id;
DROP INDEX idx_project_access_project_id_service_account_id_environment_id;

DROP TABLE project_access_scopes;
DROP TABLE project_access;