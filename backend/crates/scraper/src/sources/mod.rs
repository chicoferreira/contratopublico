pub mod base_api;
pub mod base_portal;
pub mod db;

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "contract_source", rename_all = "snake_case")]
pub enum ContractSource {
    BasePortal,
    BaseApi,
}
