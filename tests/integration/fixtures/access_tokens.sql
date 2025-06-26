-- Projects
INSERT INTO projects (id, name, description, enabled, created_at, updated_at) VALUES
('123e4567-e89b-12d3-a456-426614174000', 'testa', 'test', true, NOW(), NOW()),
('123e4567-e89b-12d3-a456-426614174001', 'testb', 'test1', false, NOW(), NOW());

-- Service Accounts
INSERT INTO service_account (id, name, email, secret, description, enabled, created_at, updated_at) VALUES
('123e4567-e89b-12d3-a456-426614174000', 'Test Account 1', 'test1@example.com', 'secret1', 'Test Description 1', true, NOW(), NOW()),
('123e4567-e89b-12d3-a456-426614174001', 'Test Account 2', 'test2@example.com', 'secret2', 'Test Description 2', true, NOW(), NOW());

-- Environments
INSERT INTO environment (id, project_id, name, description, enabled, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000001', '123e4567-e89b-12d3-a456-426614174000', 'dev', 'Development environment', true, NOW(), NOW()),
('00000000-0000-0000-0000-000000000002', '123e4567-e89b-12d3-a456-426614174000', 'prod', 'Production environment', true, NOW(), NOW());

-- Project Access
INSERT INTO project_access (id, project_id, service_account_id, environment_id, enabled, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000101', '123e4567-e89b-12d3-a456-426614174000', '123e4567-e89b-12d3-a456-426614174000', '00000000-0000-0000-0000-000000000001', true, NOW(), NOW()),
('00000000-0000-0000-0000-000000000102', '123e4567-e89b-12d3-a456-426614174000', '123e4567-e89b-12d3-a456-426614174001', '00000000-0000-0000-0000-000000000002', false, NOW(), NOW()),
('00000000-0000-0000-0000-000000000103', '123e4567-e89b-12d3-a456-426614174001', '123e4567-e89b-12d3-a456-426614174000', '00000000-0000-0000-0000-000000000001', true, NOW(), NOW()),
('00000000-0000-0000-0000-000000000104', '123e4567-e89b-12d3-a456-426614174001', '123e4567-e89b-12d3-a456-426614174001', '00000000-0000-0000-0000-000000000002', true, NOW(), NOW());

-- Access Tokens
INSERT INTO access_tokens (id, project_access_id, algorithm, token, expires_at, active, created_at, updated_at) VALUES
('11111111-1111-1111-1111-111111111111', '00000000-0000-0000-0000-000000000101', 'HS256', 'token1', '2030-01-01T00:00:00Z', true, NOW(), NOW()),
('22222222-2222-2222-2222-222222222222', '00000000-0000-0000-0000-000000000102', 'RS256', 'token2', '2030-01-02T00:00:00Z', false, NOW(), NOW()),
('33333333-3333-3333-3333-333333333333', '00000000-0000-0000-0000-000000000103', 'HS256', 'token3', '2030-01-03T00:00:00Z', true, NOW(), NOW()),
('44444444-4444-4444-4444-444444444444', '00000000-0000-0000-0000-000000000104', 'ES256', 'token4', '2030-01-04T00:00:00Z', true, NOW(), NOW()); 