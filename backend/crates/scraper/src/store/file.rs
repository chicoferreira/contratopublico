use std::path::PathBuf;

use common::Contract;

use crate::store::Store;

#[derive(Debug, Clone)]
pub struct FileStore;

fn get_folder_path(page: usize) -> PathBuf {
    let page_group = format!("group_{:05}", page / 100);
    PathBuf::new().join("contracts").join(page_group)
}

impl Store for FileStore {
    type SaveError = anyhow::Error;

    async fn save_contracts_page(
        &self,
        contracts: &[Contract],
        page: usize,
        _contracts_per_page: usize,
    ) -> Result<(), Self::SaveError> {
        let folder_path = get_folder_path(page);

        std::fs::create_dir_all(&folder_path)?;

        let file_path = folder_path.join(format!("{:07}.json", page));

        let file = std::fs::File::create(file_path)?;
        serde_json::to_writer(file, contracts)?;

        Ok(())
    }

    fn get_next_page_to_query(&self, _min: usize) -> usize {
        // let folder_path = get_folder_path(page);
        // let file_path = folder_path.join(format!("{:07}.json", page));

        // if !file_path.exists() || !file_path.is_file() {
        //     return false;
        // }

        // let contracts = std::fs::read_to_string(file_path).unwrap();
        // let contracts: Vec<Contract> = serde_json::from_str(&contracts).unwrap();

        // contracts.len() >= contracts_per_page

        todo!()
    }
}
