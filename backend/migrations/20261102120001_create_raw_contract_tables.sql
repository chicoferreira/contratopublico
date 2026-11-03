CREATE TYPE contract_source AS ENUM ('base_portal', 'base_api');

CREATE TABLE raw_contracts (
    source contract_source NOT NULL,
    contract_id BIGINT NOT NULL,
    data JSONB NOT NULL,
    fetched_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (source, contract_id)
);

CREATE INDEX idx_raw_contracts_contract_id ON raw_contracts (contract_id);

CREATE TABLE join_queue (
    contract_id BIGINT PRIMARY KEY,
    version BIGINT NOT NULL DEFAULT 0 -- Increased when queued again, so a join only dequeues the version it read
);

CREATE FUNCTION enqueue_join() RETURNS trigger AS $$
BEGIN
    INSERT INTO join_queue (contract_id)
    SELECT DISTINCT contract_id FROM changed ORDER BY contract_id
    ON CONFLICT (contract_id) DO UPDATE SET version = join_queue.version + 1;
    RETURN NULL;
END
$$ LANGUAGE plpgsql;

CREATE TRIGGER raw_contracts_inserted AFTER INSERT ON raw_contracts
REFERENCING NEW TABLE AS changed FOR EACH STATEMENT EXECUTE FUNCTION enqueue_join();

CREATE TRIGGER raw_contracts_updated AFTER UPDATE ON raw_contracts
REFERENCING NEW TABLE AS changed FOR EACH STATEMENT EXECUTE FUNCTION enqueue_join();

CREATE TABLE portal_refetch_queue (
    contract_id BIGINT PRIMARY KEY,
    version BIGINT NOT NULL DEFAULT 0 -- Increased when queued again, so a refetch only dequeues the version it read
);

CREATE FUNCTION enqueue_portal_refetch() RETURNS trigger AS $$
BEGIN
    INSERT INTO portal_refetch_queue (contract_id)
    SELECT DISTINCT contract_id FROM changed WHERE source = 'base_api' ORDER BY contract_id
    ON CONFLICT (contract_id) DO UPDATE SET version = portal_refetch_queue.version + 1;
    IF FOUND THEN
        PERFORM pg_notify('portal_refetch_queue', '');
    END IF;
    RETURN NULL;
END
$$ LANGUAGE plpgsql;

CREATE TRIGGER base_api_contracts_updated AFTER UPDATE ON raw_contracts
REFERENCING NEW TABLE AS changed FOR EACH STATEMENT EXECUTE FUNCTION enqueue_portal_refetch();
