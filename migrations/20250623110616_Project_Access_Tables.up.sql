-- Add up migration script here
CREATE TABLE project_access (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id),
    service_account_id UUID NOT NULL REFERENCES service_account(id),
    environment_id UUID NOT NULL REFERENCES environment(id),
    enabled BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_project_access_project_id_service_account_id_environment_id ON project_access (project_id, service_account_id, environment_id);


CREATE TABLE project_access_scopes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_access_id UUID NOT NULL REFERENCES project_access(id),
    scope_id UUID NOT NULL REFERENCES project_scopes(id),
    enabled BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);


CREATE UNIQUE INDEX idx_project_access_scopes_project_access_id_scope_id ON project_access_scopes (project_access_id, scope_id);
