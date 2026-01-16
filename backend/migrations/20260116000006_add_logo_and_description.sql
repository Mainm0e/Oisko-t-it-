-- Add logo_url and description to applications
ALTER TABLE applications ADD COLUMN logo_url TEXT;
ALTER TABLE applications ADD COLUMN description TEXT;
