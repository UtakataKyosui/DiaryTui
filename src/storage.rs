use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiaryStorage {
    #[serde(flatten)]
    entries: HashMap<String, String>,
}

impl DiaryStorage {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::get_storage_path()?;

        if !path.exists() {
            return Ok(Self::new());
        }

        let content = fs::read_to_string(&path)?;
        let storage: DiaryStorage = serde_json::from_str(&content)?;
        Ok(storage)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_storage_path()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(&self)?;
        fs::write(&path, content)?;
        Ok(())
    }

    pub fn get_entry(&self, date: &NaiveDate) -> Option<String> {
        let key = date.format("%Y-%m-%d").to_string();
        self.entries.get(&key).cloned()
    }

    pub fn set_entry(&mut self, date: NaiveDate, content: String) {
        let key = date.format("%Y-%m-%d").to_string();
        if content.is_empty() {
            self.entries.remove(&key);
        } else {
            self.entries.insert(key, content);
        }
    }

    pub fn has_entry(&self, date: &NaiveDate) -> bool {
        let key = date.format("%Y-%m-%d").to_string();
        self.entries.contains_key(&key)
    }

    fn get_storage_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let dirs = directories::ProjectDirs::from("", "", "DiaryTui")
            .ok_or("Failed to get project directory")?;
        let data_dir = dirs.data_dir();
        Ok(data_dir.join("diary.json"))
    }
}
