-- Projects
INSERT INTO projects (id, name, description, created_at, updated_at, enabled) VALUES ('123e4567-e89b-12d3-a456-426614174000', 'testa', 'test', NOW(), NOW(), true);
INSERT INTO projects (id, name, description, created_at, updated_at, enabled) VALUES ('123e4567-e89b-12d3-a456-426614174001', 'testb', 'test1', NOW(), NOW(), false);
INSERT INTO projects (id, name, description, created_at, updated_at, enabled) VALUES ('123e4567-e89b-12d3-a456-426614173000', 'something', 'something', NOW(), NOW(), true);
INSERT INTO projects (id, name, description, created_at, updated_at, enabled) VALUES ('123e4567-e89b-12d3-a456-426614173001', 'something1', 'something1', NOW(), NOW(), true);
INSERT INTO projects (id, name, description, created_at, updated_at, enabled) VALUES ('123e4567-e89b-12d3-a456-426614179999', 'unique_project', 'unique for test', NOW(), NOW(), true);

-- Service Accounts
INSERT INTO service_account (id, name, email, secret, description, enabled, created_at, updated_at)
VALUES 
    ('123e4567-e89b-12d3-a456-426614174000', 'Test Account 1', 'test1@example.com', 'secret1', 'Test Description 1', true, NOW(), NOW()),
    ('123e4567-e89b-12d3-a456-426614174001', 'Test Account 2', 'test2@example.com', 'secret2', 'Test Description 2', true, NOW(), NOW()),
    ('123e4567-e89b-12d3-a456-426614174002', 'Test Account 3', 'test3@example.com', 'secret3', 'Test Description 3', false, NOW(), NOW()),
    ('123e4567-e89b-12d3-a456-426614179998', 'Unique Account', 'unique@example.com', 'uniquesecret', 'Unique for test', true, NOW(), NOW());

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

-- Environments for unique project
INSERT INTO environment (id, project_id, name, description, enabled, created_at, updated_at) VALUES ('00000000-0000-0000-0000-000000009999', '123e4567-e89b-12d3-a456-426614179999', 'unique_env', 'Unique for test', true, NOW(), NOW());

-- Project access for testa project, service account 1, dev environment
INSERT INTO project_access (id, project_id, service_account_id, environment_id, enabled, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000101', '123e4567-e89b-12d3-a456-426614174000', '123e4567-e89b-12d3-a456-426614174000', '00000000-0000-0000-0000-000000000001', true, NOW(), NOW()),
('00000000-0000-0000-0000-000000000102', '123e4567-e89b-12d3-a456-426614174000', '123e4567-e89b-12d3-a456-426614174001', '00000000-0000-0000-0000-000000000002', false, NOW(), NOW()),
('00000000-0000-0000-0000-000000000103', '123e4567-e89b-12d3-a456-426614174001', '123e4567-e89b-12d3-a456-426614174002', '00000000-0000-0000-0000-000000000011', true, NOW(), NOW()),
('00000000-0000-0000-0000-000000000104', '123e4567-e89b-12d3-a456-426614174000', '123e4567-e89b-12d3-a456-426614174002', '00000000-0000-0000-0000-000000000003', true, NOW(), NOW());

-- Duplicate for unique constraint test (should not be inserted in test, but for reference)
-- ('00000000-0000-0000-0000-000000000101', '123e4567-e89b-12d3-a456-426614174000', '123e4567-e89b-12d3-a456-426614174000', '00000000-0000-0000-0000-000000000001', true, NOW(), NOW()); 