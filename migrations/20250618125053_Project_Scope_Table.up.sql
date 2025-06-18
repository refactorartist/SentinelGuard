-- Add up migration script here
CREATE TABLE IF NOT EXISTS project_scopes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id),
    scope TEXT NOT NULL,
    description TEXT NOT NULL,
    enabled BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_project_scopes_project_id_scope ON project_scopes(project_id, scope);
CREATE INDEX IF NOT EXISTS idx_project_scopes_project_id ON project_scopes(project_id);
