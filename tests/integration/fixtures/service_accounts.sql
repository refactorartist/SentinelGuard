-- Test data for service accounts
INSERT INTO service_account (id, name, email, secret, description, enabled, created_at, updated_at)
VALUES 
    ('123e4567-e89b-12d3-a456-426614174000', 'Test Account 1', 'test1@example.com', 'secret1', 'Test Description 1', true, NOW(), NOW()),
    ('123e4567-e89b-12d3-a456-426614174001', 'Test Account 2', 'test2@example.com', 'secret2', 'Test Description 2', true, NOW(), NOW()),
    ('123e4567-e89b-12d3-a456-426614174002', 'Test Account 3', 'test3@example.com', 'secret3', 'Test Description 3', false, NOW(), NOW());
