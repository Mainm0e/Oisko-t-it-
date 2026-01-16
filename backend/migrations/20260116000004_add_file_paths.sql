-- Add cv_path and cover_letter_path columns to applications table
ALTER TABLE applications ADD COLUMN cv_path TEXT;
ALTER TABLE applications ADD COLUMN cover_letter_path TEXT;
