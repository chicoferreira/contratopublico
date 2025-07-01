use std::path::PathBuf;

use crate::base_gov::Contract;

fn get_folder_path(page: usize) -> PathBuf {
    let page_group = format!("group_{:05}", page / 100);
    PathBuf::new()
        .join("contracts")
        .join(page_group)
        .join(page.to_string())
}

pub fn save_contract_to_file(contract: &Contract, page: usize) -> anyhow::Result<()> {
    let folder_path = get_folder_path(page);

    std::fs::create_dir_all(&folder_path)?;

    let file_path = folder_path.join(format!("{:07}.json", contract.id));

    let file = std::fs::File::create(file_path)?;
    serde_json::to_writer(file, contract)?;

    Ok(())
}

pub fn is_page_completed(page: usize, contracts_per_page: usize) -> bool {
    let folder_path = get_folder_path(page);

    if !folder_path.exists() || !folder_path.is_dir() {
        return false;
    }

    let number_of_files = std::fs::read_dir(&folder_path)
        .expect("folder exists")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|entry| entry.is_file() && entry.extension() == Some(std::ffi::OsStr::new("json")))
        .count();

    number_of_files >= contracts_per_page
}
