CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- only need to have one row in slot cursor, ie it needs to be singleton
CREATE TABLE IF NOT EXISTS slot_cursor(
    id smallint PRIMARY KEY DEFAULT 1,
    last_finalized_slot bigint NOT NULL DEFAULT 0,
    last_updated TIMESTAMPZ NOT NULL DEFAULT NOW(),
    CONSTRAINT singleton CHECK (id = 1))
-- table for raw blocks
CREATE TABLE IF NOT EXISTS raw_blocks(
    slot bigint PRIMARY KEY,
    block_data jsonb NOT NULL, -- parsed json data, in binary format
    block_hash varchar(88) NOT NULL,
    parent_slot bigint,
    parent_hash varchar(88),
    processed_at TIMESTAMPZ NOT NULL DEFAULT NOW(),
    processing_duration_ms integer,
    created_at TIMESTAMPZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPZ NOT NULL DEFAULT NOW())
-- dead letter queue for failed slots
CREATE TABLE IF NOT EXISTS dead_letter_queue(
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    slot bigint NOT NULL,
    error text NOT NULL,
    retry_count integer NOT NULL DEFAULT 0,
    last_retry TIMESTAMPZ,
    created_at TIMESTAMPZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPZ NOT NULL DEFAULT NOW())
-- INDEXES
CREATE INDEX IF NOT EXISTS idx_raw_blocks_parent_slot ON raw_blocks(
    parent_slot
);

CREATE INDEX IF NOT EXISTS idx_raw_blocks_created_at ON raw_blocks(created_at);

CREATE INDEX IF NOT EXISTS idx_dlq_slot ON dead_letter_queue(slot);

CREATE INDEX IF NOT EXISTS idx_dlq_retry ON dead_letter_queue(retry_count, last_retry);

CREATE INDEX IF NOT EXISTS idx_dlq_created_at ON dead_letter_queue(created_at);

-- function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
    RETURNS TRIGGER
    AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$
LANGUAGE 'plpgsql';

-- Create triggers for updated_at
DROP TRIGGER IF EXISTS update_raw_blocks_updated_at ON raw_blocks;

CREATE TRIGGER update_raw_blocks_updated_at
    BEFORE UPDATE ON raw_blocks
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

