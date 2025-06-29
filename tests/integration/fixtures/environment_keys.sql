-- Projects Fixture
INSERT INTO projects (id, name, description, enabled, created_at, updated_at) VALUES
('11111111-1111-1111-1111-111111111111', 'Test Project', 'A test project', true, '2025-06-16T03:48:22.000Z', '2025-06-16T03:48:22.000Z');

-- Environments Fixture
INSERT INTO environment (id, project_id, name, description, enabled, created_at, updated_at) VALUES
('123e4567-e89b-12d3-a456-426614174000', '11111111-1111-1111-1111-111111111111', 'Test Environment', 'A test environment', true, '2025-06-16T03:48:22.000Z', '2025-06-16T03:48:22.000Z');

-- Environment Keys Fixture
INSERT INTO environment_key (id, environment_id, algorithm, key, active, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000001', '123e4567-e89b-12d3-a456-426614174000', 'HS256', 'key1', true, '2025-06-16T03:48:22.000Z', '2025-06-16T03:48:22.000Z'),
('00000000-0000-0000-0000-000000000002', '123e4567-e89b-12d3-a456-426614174000', 'RS256', 'key2', false, '2025-06-16T03:48:22.000Z', '2025-06-16T03:48:22.000Z'); 