use std::{
    fs::OpenOptions,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use anyhow::Context;

use crate::store::Store;

#[derive(Debug)]
pub struct MeilisearchStore {
    inner: Arc<MeilisearchInner>,
}

#[derive(Debug)]
struct MeilisearchInner {
    client: Arc<meilisearch_sdk::client::Client>,
    saved_pages: Mutex<Vec<usize>>,
    saved_pages_path: PathBuf,
}

impl MeilisearchStore {
    fn get_file(path: &Path) -> anyhow::Result<std::fs::File> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).context("Failed to create folder")?;
        }

        OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(path)
            .context("Failed to open saved pages file")
    }

    pub fn new(
        client: Arc<meilisearch_sdk::client::Client>,
        saved_pages_path: PathBuf,
    ) -> anyhow::Result<Self> {
        let file = Self::get_file(&saved_pages_path)?;

        let mut saved_pages: Vec<usize> = serde_json::from_reader(file).unwrap_or(vec![]);
        saved_pages.dedup();
        saved_pages.sort();

        let saved_pages = Mutex::new(saved_pages);

        Ok(MeilisearchStore {
            inner: Arc::new(MeilisearchInner {
                client,
                saved_pages,
                saved_pages_path,
            }),
        })
    }

    pub fn save_page_as_completed(&self, page: usize) -> anyhow::Result<()> {
        let mut saved_pages = self.inner.saved_pages.lock().unwrap();
        saved_pages.push(page);
        saved_pages.sort();

        let file = Self::get_file(&self.inner.saved_pages_path)?;

        serde_json::to_writer(file, &*saved_pages).context("Failed to write saved pages")?;

        Ok(())
    }
}

impl Clone for MeilisearchStore {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl Store for MeilisearchStore {
    type SaveError = anyhow::Error;

    async fn save_contracts_page(
        &self,
        contracts: &[common::Contract],
        page: usize,
        contracts_per_page: usize,
    ) -> Result<(), Self::SaveError> {
        let index = self.inner.client.index("contracts");

        index
            .add_documents(contracts, Some("id"))
            .await
            .context("Failed to save contracts")?;

        if contracts.len() == contracts_per_page {
            self.save_page_as_completed(page)?;
        }

        Ok(())
    }

    fn get_next_page_to_query(&self, min: usize) -> usize {
        let saved_pages = self.inner.saved_pages.lock().unwrap();
        for page in min.. {
            // TODO: this can be done comparing the previous to the next and check when there is a gap, deal with it later
            if saved_pages.binary_search(&page).is_err() {
                return page;
            }
        }
        unreachable!()
    }
}
