CREATE TABLE spaces (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
  name VARCHAR(50) NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

COMMENT ON TABLE spaces IS 'スペース';
COMMENT ON COLUMN spaces.name IS 'スペース名';

CREATE TRIGGER update_spaces_updated_at
BEFORE UPDATE ON spaces
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

alter table spaces enable row level security;


