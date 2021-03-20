-- Add migration script here

ALTER TABLE tags
ADD COLUMN color INTEGER CHECK (color >= 0 AND color <= 16777215)