-- Project scopes for testa project (enabled project)
INSERT INTO project_scopes (id, project_id, scope, description, enabled, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000001', '123e4567-e89b-12d3-a456-426614174000', 'testa:read', 'Read access to testa project', true, NOW(), NOW()),
('00000000-0000-0000-0000-000000000002', '123e4567-e89b-12d3-a456-426614174000', 'testa:write', 'Write access to testa project', true, NOW(), NOW()),
('00000000-0000-0000-0000-000000000003', '123e4567-e89b-12d3-a456-426614174000', 'testa:admin', 'Admin access to testa project', true, NOW(), NOW());

-- Project scopes for testb project (disabled project)
INSERT INTO project_scopes (id, project_id, scope, description, enabled, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000011', '123e4567-e89b-12d3-a456-426614174001', 'testb:read', 'Read access to testb project', false, NOW(), NOW()),
('00000000-0000-0000-0000-000000000012', '123e4567-e89b-12d3-a456-426614174001', 'testb:write', 'Write access to testb project', true, NOW(), NOW());

-- Project scopes for something project
INSERT INTO project_scopes (id, project_id, scope, description, enabled, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000021', '123e4567-e89b-12d3-a456-426614173000', 'something:read', 'Read access to something project', true, NOW(), NOW()),
('00000000-0000-0000-0000-000000000022', '123e4567-e89b-12d3-a456-426614173000', 'something:write', 'Write access to something project', true, NOW(), NOW()),
('00000000-0000-0000-0000-000000000023', '123e4567-e89b-12d3-a456-426614173000', 'something:delete', 'Delete access to something project', false, NOW(), NOW());

-- Project scopes for something1 project
INSERT INTO project_scopes (id, project_id, scope, description, enabled, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000031', '123e4567-e89b-12d3-a456-426614173001', 'something1:read', 'Read access to something1 project', true, NOW(), NOW()),
('00000000-0000-0000-0000-000000000032', '123e4567-e89b-12d3-a456-426614173001', 'something1:write', 'Write access to something1 project', true, NOW(), NOW()),
('00000000-0000-0000-0000-000000000033', '123e4567-e89b-12d3-a456-426614173001', 'something1:admin', 'Admin access to something1 project', true, NOW(), NOW());

-- Additional scopes for testa project with different patterns
INSERT INTO project_scopes (id, project_id, scope, description, enabled, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000041', '123e4567-e89b-12d3-a456-426614174000', 'testa:api:read', 'API read access to testa project', true, NOW(), NOW()),
('00000000-0000-0000-0000-000000000042', '123e4567-e89b-12d3-a456-426614174000', 'testa:api:write', 'API write access to testa project', true, NOW(), NOW()),
('00000000-0000-0000-0000-000000000043', '123e4567-e89b-12d3-a456-426614174000', 'testa:db:read', 'Database read access to testa project', false, NOW(), NOW());