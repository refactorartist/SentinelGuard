-- Migration for creating the 'project' table
CREATE TABLE IF NOT EXISTS projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    enabled BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Optional: Indexes for fast lookup
CREATE INDEX IF NOT EXISTS idx_project_name ON projects(name);
CREATE INDEX IF NOT EXISTS idx_project_enabled ON projects(enabled);
