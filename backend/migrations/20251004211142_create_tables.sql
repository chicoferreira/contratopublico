CREATE TABLE IF NOT EXISTS cpv (
    code TEXT PRIMARY KEY,
    designation TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS entities (
    id BIGINT PRIMARY KEY,
    nif TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS documents (
    id BIGINT PRIMARY KEY,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS contracts (
    id BIGINT PRIMARY KEY,
    contracting_procedure_type TEXT NOT NULL,
    publication_date DATE NOT NULL,
    signing_date DATE,
    ccp BOOLEAN NOT NULL,
    object_brief_description TEXT NOT NULL,
    initial_contractual_price BIGINT NOT NULL, -- Currency in cents
    description TEXT,
    regime TEXT,
    contract_status TEXT,
    non_written_contract_justification_types TEXT NOT NULL,
    contract_types TEXT NOT NULL,
    execution_deadline_days INTEGER NOT NULL,
    execution_place TEXT NOT NULL,
    contract_fundamentation_type TEXT NOT NULL,
    contracting_procedure_url TEXT,
    announcement_id BIGINT,
    direct_award_fundamentation_type TEXT NOT NULL,
    observations TEXT,
    end_of_contract_type TEXT,
    close_date DATE,
    total_effective_price BIGINT, -- Currency in cents
    causes_deadline_change TEXT,
    causes_price_change TEXT
);

-- Relationship tables for many-to-many fields on Contract
CREATE TABLE IF NOT EXISTS contract_contracting (
    contract_id BIGINT NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    entity_id BIGINT NOT NULL REFERENCES entities(id),
    description TEXT NOT NULL,
    PRIMARY KEY (contract_id, entity_id)
);
CREATE INDEX IF NOT EXISTS idx_contract_contracting_entity ON contract_contracting(entity_id);

CREATE TABLE IF NOT EXISTS contract_contracted (
    contract_id BIGINT NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    entity_id BIGINT NOT NULL REFERENCES entities(id),
    description TEXT NOT NULL,
    PRIMARY KEY (contract_id, entity_id)
);
CREATE INDEX IF NOT EXISTS idx_contract_contracted_entity ON contract_contracted(entity_id);

CREATE TABLE IF NOT EXISTS contract_contestants (
    contract_id BIGINT NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    entity_id BIGINT NOT NULL REFERENCES entities(id),
    description TEXT NOT NULL,
    PRIMARY KEY (contract_id, entity_id)
);
CREATE INDEX IF NOT EXISTS idx_contract_contestants_entity ON contract_contestants(entity_id);

CREATE TABLE IF NOT EXISTS contract_invitees (
    contract_id BIGINT NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    entity_id BIGINT NOT NULL REFERENCES entities(id),
    description TEXT NOT NULL,
    PRIMARY KEY (contract_id, entity_id)
);
CREATE INDEX IF NOT EXISTS idx_contract_invitees_entity ON contract_invitees(entity_id);

CREATE TABLE IF NOT EXISTS contract_documents (
    contract_id BIGINT NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    document_id BIGINT NOT NULL REFERENCES documents(id),
    PRIMARY KEY (contract_id, document_id)
);
CREATE INDEX IF NOT EXISTS idx_contract_documents_document ON contract_documents(document_id);

CREATE TABLE IF NOT EXISTS contract_cpvs (
    contract_id BIGINT NOT NULL REFERENCES contracts(id) ON DELETE CASCADE,
    cpv_code TEXT NOT NULL REFERENCES cpv(code),
    PRIMARY KEY (contract_id, cpv_code)
);
CREATE INDEX IF NOT EXISTS idx_contract_cpvs_contract ON contract_cpvs(contract_id);
CREATE INDEX IF NOT EXISTS idx_contract_cpvs_cpv ON contract_cpvs(cpv_code);
