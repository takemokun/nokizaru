DROP TRIGGER IF EXISTS update_admins_updated_at ON admins;

DROP TABLE admins;

DROP FUNCTION IF EXISTS update_updated_at_column();
