CREATE TYPE contract_type AS ENUM (
    'goods_acquisition',
    'services_acquisition',
    'public_works_concession',
    'public_services_concession',
    'public_works',
    'goods_lease',
    'partnership',
    'other'
);

CREATE TYPE procedure_type AS ENUM (
    'direct_award',
    'direct_award_rural_fires',
    'simplified_direct_award',
    'simplified_direct_award_special_measures',
    'prior_consultation',
    'prior_consultation_rural_fires',
    'simplified_prior_consultation',
    'public_tender',
    'simplified_public_tender',
    'restricted_tender',
    'simplified_restricted_tender',
    'negotiated_procedure',
    'competitive_dialogue',
    'innovation_partnership',
    'framework_agreement_art_258',
    'framework_agreement_art_259',
    'goods_provision',
    'social_and_other_specific_services',
    'simplified_design_contest',
    'simplified_ideas_contest',
    'special_sectors_exemption',
    'excluded_contracting'
);

CREATE TABLE entities (
    id BIGINT PRIMARY KEY,
    nif TEXT NOT NULL
);

CREATE TABLE documents (
    id BIGINT PRIMARY KEY,
    description TEXT NOT NULL
);

CREATE TABLE contracts (
    id BIGINT PRIMARY KEY,
    procedure_type procedure_type,
    publication_date DATE NOT NULL,
    signing_date DATE,
    object_brief_description TEXT NOT NULL,
    initial_contractual_price BIGINT NOT NULL, -- Currency in cents
    description TEXT,
    cpvs TEXT[] NOT NULL,
    regime TEXT,
    contract_status TEXT,
    non_written_contract_justification_types TEXT[] NOT NULL,
    contract_types contract_type[] NOT NULL,
    execution_deadline_days INTEGER NOT NULL,
    execution_places TEXT[] NOT NULL,
    contract_fundamentation_type TEXT NOT NULL,
    contracting_procedure_url TEXT,
    announcement_id BIGINT,
    direct_award_fundamentation_type TEXT,
    observations TEXT,
    end_of_contract_type TEXT,
    close_date DATE,
    total_effective_price BIGINT, -- Currency in cents
    causes_deadline_change TEXT,
    causes_price_change TEXT,
    estimated_base_price BIGINT, -- Currency in cents
    award_decision_date DATE,
    framework_agreement_description TEXT,
    special_measures TEXT,
    ambient_criteria BOOLEAN NOT NULL
);

CREATE INDEX idx_contracts_pubdate_cover
    ON contracts (publication_date)
    INCLUDE (initial_contractual_price);

-- Relationship tables for many-to-many fields on Contract
CREATE TABLE contract_contracting (
    contract_id BIGINT NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    entity_id BIGINT NOT NULL REFERENCES entities(id),
    description TEXT NOT NULL,
    PRIMARY KEY (contract_id, entity_id)
);
CREATE INDEX idx_contract_contracting_entity ON contract_contracting(entity_id);

CREATE TABLE contract_contracted (
    contract_id BIGINT NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    entity_id BIGINT NOT NULL REFERENCES entities(id),
    description TEXT NOT NULL,
    PRIMARY KEY (contract_id, entity_id)
);
CREATE INDEX idx_contract_contracted_entity ON contract_contracted(entity_id);

CREATE TABLE contract_contestants (
    contract_id BIGINT NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    entity_id BIGINT NOT NULL REFERENCES entities(id),
    description TEXT NOT NULL,
    PRIMARY KEY (contract_id, entity_id)
);
CREATE INDEX idx_contract_contestants_entity ON contract_contestants(entity_id);

CREATE TABLE contract_invitees (
    contract_id BIGINT NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    entity_id BIGINT NOT NULL REFERENCES entities(id),
    description TEXT NOT NULL,
    PRIMARY KEY (contract_id, entity_id)
);
CREATE INDEX idx_contract_invitees_entity ON contract_invitees(entity_id);

CREATE TABLE contract_documents (
    contract_id BIGINT NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    document_id BIGINT NOT NULL REFERENCES documents(id),
    PRIMARY KEY (contract_id, document_id)
);
CREATE INDEX idx_contract_documents_document ON contract_documents(document_id);

CREATE MATERIALIZED VIEW contract_spent_daily AS
SELECT
    publication_date::date AS date,
    COUNT(*)::BIGINT AS count,
    COALESCE(SUM(initial_contractual_price) FILTER (WHERE initial_contractual_price > 0), 0)::BIGINT AS amount
FROM contracts
GROUP BY 1;

CREATE UNIQUE INDEX contract_daily_pk ON contract_spent_daily (date);
