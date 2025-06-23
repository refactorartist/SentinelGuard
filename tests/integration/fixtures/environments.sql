-- Environments for testa project (enabled project)
INSERT INTO environment (id, project_id, name, description, enabled, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000001', '123e4567-e89b-12d3-a456-426614174000', 'dev', 'Development environment', true, NOW(), NOW()),
('00000000-0000-0000-0000-000000000002', '123e4567-e89b-12d3-a456-426614174000', 'staging', 'Staging environment', false, NOW(), NOW()),
('00000000-0000-0000-0000-000000000003', '123e4567-e89b-12d3-a456-426614174000', 'prod', 'Production environment', true, NOW(), NOW());

-- Environments for testb project (disabled project)
INSERT INTO environment (id, project_id, name, description, enabled, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000011', '123e4567-e89b-12d3-a456-426614174001', 'dev', 'Development environment', false, NOW(), NOW()),
('00000000-0000-0000-0000-000000000012', '123e4567-e89b-12d3-a456-426614174001', 'prod', 'Production environment', true, NOW(), NOW());

-- Environments for something project
INSERT INTO environment (id, project_id, name, description, enabled, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000021', '123e4567-e89b-12d3-a456-426614173000', 'dev', 'Development environment', true, NOW(), NOW()),
('00000000-0000-0000-0000-000000000022', '123e4567-e89b-12d3-a456-426614173000', 'prod', 'Production environment', false, NOW(), NOW());

-- Environments for something1 project
INSERT INTO environment (id, project_id, name, description, enabled, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000031', '123e4567-e89b-12d3-a456-426614173001', 'dev', 'Development environment', true, NOW(), NOW()),
('00000000-0000-0000-0000-000000000032', '123e4567-e89b-12d3-a456-426614173001', 'staging', 'Staging environment', false, NOW(), NOW()),
('00000000-0000-0000-0000-000000000033', '123e4567-e89b-12d3-a456-426614173001', 'prod', 'Production environment', true, NOW(), NOW());
