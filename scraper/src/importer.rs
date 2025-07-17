use anyhow::Context;
use common::Contract;
use log::{error, info};
use meilisearch_sdk::client::Client;

pub async fn import_contracts_to_meilisearch(
    meilisearch_url: String,
    meilisearch_api_key: Option<String>,
) -> anyhow::Result<()> {
    let client =
        Client::new(&meilisearch_url, meilisearch_api_key).context("Couldn't create client")?;

    let index = client.index("contracts");

    let contracts_dir = "contracts";
    let entries = std::fs::read_dir(contracts_dir).context("Failed to read contracts directory")?;

    for entry in entries {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path.is_dir() {
            let json_files = std::fs::read_dir(&path).context("Failed to read subdirectory")?;

            for json_file in json_files {
                let json_file = json_file.context("Failed to read file entry")?;
                let json_path = json_file.path();

                if json_path.is_file()
                    && json_path.extension().and_then(|s| s.to_str()) == Some("json")
                {
                    let content =
                        std::fs::read_to_string(&json_path).context("Failed to read JSON file")?;

                    let contracts: Vec<Contract> =
                        serde_json::from_str(&content).context("Failed to parse JSON")?;
                    
                    let index = index.clone();

                    tokio::spawn(async move {
                        if let Err(err) = index
                            .add_or_replace(&contracts, Some("id"))
                            .await
                            .context("Failed to add document to index")
                        {
                            error!(
                                "Failed to add contracts in file {}: {}",
                                json_path.display(),
                                err
                            );
                        } else {
                            info!("Added contracts in file {}", json_path.display());
                        }
                    });
                }
            }
        }
    }

    Ok(())
}
