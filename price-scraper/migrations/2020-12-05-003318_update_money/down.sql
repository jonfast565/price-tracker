-- This file should undo anything in `up.sql`

ALTER TABLE prices ALTER COLUMN price SET DATA TYPE MONEY;