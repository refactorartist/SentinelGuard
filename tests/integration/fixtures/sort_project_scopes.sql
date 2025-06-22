-- Project scopes for sorting tests
INSERT INTO project_scopes (id, project_id, scope, description, enabled, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000101', '123e4567-e89b-12d3-a456-426614174000', 'scope:a', 'Scope A Description', true, NOW(), NOW()),
('00000000-0000-0000-0000-000000000102', '123e4567-e89b-12d3-a456-426614174000', 'scope:b', 'Scope B Description', true, NOW(), NOW()),
('00000000-0000-0000-0000-000000000103', '123e4567-e89b-12d3-a456-426614174000', 'scope:c', 'Scope C Description', true, NOW(), NOW());

-- Project scopes for different projects (for project_id sorting)
INSERT INTO project_scopes (id, project_id, scope, description, enabled, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000201', '123e4567-e89b-12d3-a456-426614173000', 'project1:scope', 'Project 1 Scope', true, NOW(), NOW()),
('00000000-0000-0000-0000-000000000202', '123e4567-e89b-12d3-a456-426614173001', 'project2:scope', 'Project 2 Scope', true, NOW(), NOW()),
('00000000-0000-0000-0000-000000000203', '123e4567-e89b-12d3-a456-426614174001', 'project3:scope', 'Project 3 Scope', true, NOW(), NOW());
