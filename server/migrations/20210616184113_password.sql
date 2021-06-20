ALTER TABLE library ADD COLUMN password text;
UPDATE library SET password = 'pepperrat';
ALTER TABLE library ALTER COLUMN password SET NOT NULL;
