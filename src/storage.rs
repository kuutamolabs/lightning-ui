use anyhow::{bail, Result};
use log::error;
use web_sys::Storage;

pub struct LocalStorage {
    storage: Storage,
}

impl LocalStorage {
    pub fn new() -> LocalStorage {
        let storage = web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .expect("user has not enabled localStorage");

        LocalStorage { storage }
    }

    pub fn get_macaroon(&self) -> Option<String> {
        self.get("macaroon")
    }

    pub fn set_macaroon(&self, macaroon: &str) -> Result<()> {
        self.set("macaroon", macaroon)
    }

    pub fn get_url(&self) -> Option<String> {
        self.get("url")
    }

    pub fn set_url(&self, url: &str) -> Result<()> {
        self.set("url", url)
    }

    fn set(&self, key: &str, value: &str) -> Result<()> {
        if let Err(e) = self.storage.set(key, value) {
            if let Some(s) = e.as_string() {
                error!("{}", s);
            }
            bail!("Error setting value for key {}", key);
        }
        Ok(())
    }

    fn get(&self, key: &str) -> Option<String> {
        let value = self.storage.get(key);
        if let Err(e) = value {
            if let Some(s) = e.as_string() {
                error!("{}", s);
            } else {
                error!("Error getting value for key {}", key);
            }
            None
        } else {
            value.unwrap()
        }
    }
}
