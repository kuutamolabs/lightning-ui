use std::cell::RefCell;

use anyhow::{anyhow, Result};
use api::GetInfo;
use log::debug;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};

pub static DEFAULT_URL: &str = "http://127.0.0.1:2244";

pub struct Api {
    client: RefCell<Client>,
    url: RefCell<String>,
}

impl Api {
    pub fn new() -> Result<Api> {
        let client = reqwest::ClientBuilder::new().build()?;
        let url = "http://127.0.0.1:40002".to_string();
        Ok(Api {
            client: RefCell::new(client),
            url: RefCell::new(url),
        })
    }

    pub fn connect(&self, url: String, macaroon: String) {
        debug!("Connecting to {}", url);
        let mut headers = HeaderMap::new();
        headers.insert("macaroon", HeaderValue::from_str(&macaroon).unwrap());
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();
        self.client.replace(client);
        self.url.replace(url);
    }

    pub async fn get_info(&self) -> Result<GetInfo> {
        debug!("Calling API getinfo");

        let request = self
            .client
            .borrow()
            .get(format!("{}/v1/getinfo", self.url.borrow()))
            .send();

        let result = request
            .await
            .map_err(|_| anyhow!("Connection Refused"))?
            .error_for_status()?;

        let get_info: GetInfo = result.json().await?;
        Ok(get_info)
    }
}
