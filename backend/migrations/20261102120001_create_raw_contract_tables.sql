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

CREATE TRIGGER raw_contracts_deleted AFTER DELETE ON raw_contracts
REFERENCING OLD TABLE AS changed FOR EACH STATEMENT EXECUTE FUNCTION enqueue_join();

CREATE FUNCTION notify_queue() RETURNS trigger AS $$
BEGIN
    PERFORM pg_notify(TG_TABLE_NAME, '');
    RETURN NULL;
END
$$ LANGUAGE plpgsql;

CREATE TRIGGER join_queue_changed AFTER INSERT OR UPDATE ON join_queue
FOR EACH STATEMENT EXECUTE FUNCTION notify_queue();

CREATE TABLE portal_refetch_queue (
    contract_id BIGINT PRIMARY KEY,
    version BIGINT NOT NULL DEFAULT 0 -- Increased when queued again, so a refetch only dequeues the version it read
);

CREATE FUNCTION enqueue_portal_refetch() RETURNS trigger AS $$
BEGIN
    INSERT INTO portal_refetch_queue (contract_id)
    SELECT DISTINCT contract_id FROM changed WHERE source = 'base_api' ORDER BY contract_id
    ON CONFLICT (contract_id) DO UPDATE SET version = portal_refetch_queue.version + 1;
    RETURN NULL;
END
$$ LANGUAGE plpgsql;

CREATE TRIGGER base_api_contracts_updated AFTER UPDATE ON raw_contracts
REFERENCING NEW TABLE AS changed FOR EACH STATEMENT EXECUTE FUNCTION enqueue_portal_refetch();

CREATE TRIGGER base_api_contracts_deleted AFTER DELETE ON raw_contracts
REFERENCING OLD TABLE AS changed FOR EACH STATEMENT EXECUTE FUNCTION enqueue_portal_refetch();

CREATE TRIGGER portal_refetch_queue_changed AFTER INSERT OR UPDATE ON portal_refetch_queue
FOR EACH STATEMENT EXECUTE FUNCTION notify_queue();
