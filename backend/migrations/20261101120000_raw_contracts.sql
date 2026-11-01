CREATE TYPE contract_source AS ENUM ('base_portal', 'base_api');

CREATE TABLE raw_contracts (
    source contract_source NOT NULL,
    contract_id BIGINT NOT NULL,
    data JSONB NOT NULL,
    fetched_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (source, contract_id)
);
