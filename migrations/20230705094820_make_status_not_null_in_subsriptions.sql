-- Add migration script here

-- Wrapped in transaction to ensure it is atomic

BEGIN;

-- update historical entries

UPDATE subscriptions
SET
    status = 'confirmed'
WHERE status IS NULL;

-- make 'status' mandatory

ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;

COMMIT;