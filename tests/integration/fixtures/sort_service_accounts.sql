-- Test data for sorting service accounts
INSERT INTO service_account (id, name, email, secret, description, enabled, created_at, updated_at)
VALUES 
    ('123e4567-e89b-12d3-a456-426614174010', 'Account B', 'b@example.com', 'secret-b', 'Description B', true, NOW(), NOW()),
    ('123e4567-e89b-12d3-a456-426614174011', 'Account A', 'a@example.com', 'secret-a', 'Description A', true, NOW(), NOW()),
    ('123e4567-e89b-12d3-a456-426614174012', 'Account C', 'c@example.com', 'secret-c', 'Description C', true, NOW(), NOW());
