-- 初期セットアップ start

-- UUID自動生成用の拡張機能をインストール
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- UUID v7 generation function
-- Based on https://github.com/Betterment/postgresql-uuid-generate-v7
-- Creates time-ordered UUIDs for better database performance
CREATE OR REPLACE FUNCTION uuid_generate_v7()
RETURNS uuid
LANGUAGE plpgsql
VOLATILE
AS $$
DECLARE
    unix_ts_ms BIGINT;
    uuid_bytes BYTEA;
    random_part BYTEA;
BEGIN
    -- Get current Unix timestamp in milliseconds (48 bits)
    unix_ts_ms = FLOOR(EXTRACT(EPOCH FROM clock_timestamp()) * 1000);

    -- Generate 10 bytes of random data
    random_part = gen_random_bytes(10);

    -- Construct UUID v7 (total 16 bytes):
    -- 6 bytes: timestamp (48 bits)
    -- 2 bytes: version + random (16 bits)
    -- 8 bytes: variant + random (64 bits)
    uuid_bytes =
        -- Timestamp (48 bits / 6 bytes)
        substring(int8send(unix_ts_ms) from 3 for 6) ||
        -- Version 7 (4 bits) + random (12 bits) = 2 bytes
        set_byte(
            substring(random_part from 1 for 2),
            0,
            (get_byte(random_part, 0) & 15) | 112  -- 0x70 = version 7
        ) ||
        -- Variant bits (2 bits) + random (62 bits) = 8 bytes
        set_byte(
            substring(random_part from 3 for 8),
            0,
            (get_byte(random_part, 2) & 63) | 128  -- 0x80 = variant 10
        );

    RETURN encode(uuid_bytes, 'hex')::uuid;
END;
$$;

-- updated_at を自動更新するトリガー用関数を作成
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
