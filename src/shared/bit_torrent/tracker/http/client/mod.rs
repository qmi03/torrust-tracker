pub mod requests;
pub mod responses;

use std::net::IpAddr;

use requests::announce::{self, Query};
use requests::scrape;
use reqwest::{Client as ReqwestClient, Response, Url};

use crate::core::auth::Key;

/// HTTP Tracker Client
pub struct Client {
    base_url: Url,
    reqwest: ReqwestClient,
    key: Option<Key>,
}

/// URL components in this context:
///
/// ```text
/// http://127.0.0.1:62304/announce/YZ....rJ?info_hash=%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22
/// \_____________________/\_______________/ \__________________________________________________________/
///            |                   |                                    |
///         base url              path                                query
/// ```
impl Client {
    /// # Panics
    ///
    /// This method fails if the client builder fails.
    #[must_use]
    pub fn new(base_url: Url) -> Self {
        Self {
            base_url,
            reqwest: reqwest::Client::builder().build().unwrap(),
            key: None,
        }
    }

    /// Creates the new client binding it to an specific local address.
    ///
    /// # Panics
    ///
    /// This method fails if the client builder fails.
    #[must_use]
    pub fn bind(base_url: Url, local_address: IpAddr) -> Self {
        Self {
            base_url,
            reqwest: reqwest::Client::builder().local_address(local_address).build().unwrap(),
            key: None,
        }
    }

    /// # Panics
    ///
    /// This method fails if the client builder fails.
    #[must_use]
    pub fn authenticated(base_url: Url, key: Key) -> Self {
        Self {
            base_url,
            reqwest: reqwest::Client::builder().build().unwrap(),
            key: Some(key),
        }
    }

    pub async fn announce(&self, query: &announce::Query) -> Response {
        self.get(&self.build_announce_path_and_query(query)).await
    }

    pub async fn scrape(&self, query: &scrape::Query) -> Response {
        self.get(&self.build_scrape_path_and_query(query)).await
    }

    pub async fn announce_with_header(&self, query: &Query, key: &str, value: &str) -> Response {
        self.get_with_header(&self.build_announce_path_and_query(query), key, value)
            .await
    }

    pub async fn health_check(&self) -> Response {
        self.get(&self.build_path("health_check")).await
    }

    /// # Panics
    ///
    /// This method fails if there was an error while sending request.
    pub async fn get(&self, path: &str) -> Response {
        self.reqwest.get(self.build_url(path)).send().await.unwrap()
    }

    /// # Panics
    ///
    /// This method fails if there was an error while sending request.
    pub async fn get_with_header(&self, path: &str, key: &str, value: &str) -> Response {
        self.reqwest
            .get(self.build_url(path))
            .header(key, value)
            .send()
            .await
            .unwrap()
    }

    fn build_announce_path_and_query(&self, query: &announce::Query) -> String {
        format!("{}?{query}", self.build_path("announce"))
    }

    fn build_scrape_path_and_query(&self, query: &scrape::Query) -> String {
        format!("{}?{query}", self.build_path("scrape"))
    }

    fn build_path(&self, path: &str) -> String {
        match &self.key {
            Some(key) => format!("{path}/{key}"),
            None => path.to_string(),
        }
    }

    fn build_url(&self, path: &str) -> String {
        let base_url = self.base_url();
        format!("{base_url}{path}")
    }

    fn base_url(&self) -> String {
        self.base_url.to_string()
    }
}
