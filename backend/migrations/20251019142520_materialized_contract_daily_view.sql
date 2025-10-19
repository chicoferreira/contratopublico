CREATE INDEX IF NOT EXISTS idx_contracts_pubdate_cover
  ON contracts (publication_date)
  INCLUDE (initial_contractual_price);

CREATE MATERIALIZED VIEW IF NOT EXISTS contract_spent_daily AS
SELECT
  publication_date::date AS date,
  COUNT(*)::BIGINT AS count,
  SUM(initial_contractual_price)::BIGINT AS amount
FROM contracts
GROUP BY 1;

CREATE UNIQUE INDEX IF NOT EXISTS contract_daily_pk ON contract_spent_daily (date);
