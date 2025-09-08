use std::{
    collections::{HashMap, HashSet},
    fs::{File, OpenOptions},
    path::{Path, PathBuf},
    sync::Mutex,
};

use anyhow::Context;
use common::Contract;
use serde::{Deserialize, Serialize};

use crate::store::rangeset::RangeSet;

pub mod rangeset;

pub struct Store {
    client: meilisearch_sdk::client::Client,
    scrape_progress: Mutex<ScrapeProgress>,
    path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct ScrapeProgress {
    /// An optimized set of the pages that have been scraped and saved
    saved_pages: RangeSet<usize>,
    /// A map of page (that have not been completely scraped yet) numbers
    /// to the set of contract ids that have been scraped and saved
    pending_pages: HashMap<usize, HashSet<usize>>,
}

impl ScrapeProgress {
    fn update(&mut self, page: usize, contracts_per_page: usize, id: usize) {
        if self.saved_pages.contains(&page) {
            // already saved
            return;
        }

        let page_progress = self.pending_pages.entry(page).or_default();
        page_progress.insert(id);
        if page_progress.len() >= contracts_per_page {
            self.saved_pages.insert(page);
            self.pending_pages.remove(&page);
        }
    }
}

impl Store {
    fn load_progress(path: &Path) -> anyhow::Result<ScrapeProgress> {
        if !path.exists() {
            return Ok(ScrapeProgress::default());
        }

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).context("Failed to create folder")?;
        }

        let file = File::open(path).context("Failed to open saved pages file")?;
        serde_json::from_reader(file).context("Failed to deserialize progress")
    }

    fn create_file_for_writing(path: &Path) -> anyhow::Result<File> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).context("Failed to create folder")?;
        }

        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .context("Failed to open saved pages file")
    }

    pub fn new(client: meilisearch_sdk::client::Client, path: PathBuf) -> anyhow::Result<Self> {
        let scrape_progress = Self::load_progress(&path).context("Failed to load progress")?;

        Ok(Self {
            client,
            scrape_progress: Mutex::new(scrape_progress),
            path,
        })
    }

    pub async fn already_exists(&self, id: usize, page: usize) -> bool {
        let scrape_progress = self.scrape_progress.lock().unwrap();

        scrape_progress.saved_pages.contains(&page)
            || scrape_progress
                .pending_pages
                .get(&page)
                .map_or(false, |entry| entry.contains(&id))
    }

    pub async fn save_contract(
        &self,
        contract: Contract,
        page: usize,
        contracts_per_page: usize,
    ) -> anyhow::Result<()> {
        let index = self.client.index("contracts");
        let id = contract.id;

        index
            .add_documents(&[contract], Some("id"))
            .await
            .context("Failed to save contract")?;

        let mut scrape_progress = self.scrape_progress.lock().unwrap();
        scrape_progress.update(page, contracts_per_page, id);

        let file = Self::create_file_for_writing(&self.path)?;
        serde_json::to_writer(file, &*scrape_progress).context("Failed to write saved pages")?;

        Ok(())
    }

    pub fn get_next_page_to_query(&self, current_page: usize) -> usize {
        let progress = self.scrape_progress.lock().unwrap();

        let min_pending_page = progress
            .pending_pages
            .keys()
            .filter(|&&page| page >= current_page)
            .min()
            .copied();

        let first_missing = progress.saved_pages.get_first_missing(current_page);

        min_pending_page.map_or(first_missing, |min_pending| min_pending.min(first_missing))
    }
}
