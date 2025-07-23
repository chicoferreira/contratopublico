use std::fmt::Debug;

use common::Contract;

pub mod file;
pub mod meilisearch;

pub trait Store: Clone + Sync + Send {
    type SaveError: Debug;

    fn save_contracts_page(
        &self,
        contracts: &[Contract],
        page: usize,
        contracts_per_page: usize,
    ) -> impl std::future::Future<Output = Result<(), Self::SaveError>> + Send;

    fn get_next_page_to_query(&self, min: usize) -> usize;
}
