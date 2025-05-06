use crate::{error::Result, types::{config::Configuration, events::Event}};
use reqwest::header;
use tokio::sync::mpsc::Sender;

const ADDR: &str = "http://localhost:8384/rest";

/// A `ClientBuilder` can be used to create a `Client` with custom configuration.
#[must_use]
pub struct ClientBuilder {
    base_url: Option<String>,
    api_key: String,
}

impl ClientBuilder {
    /// Constructs a new `ClientBuilder`.
    /// This is the same as `Client::builder()`.
    ///
    /// The API can either be generated in the GUI of Syncthing or set
    /// in the configuration file under `configuration/gui/apikey`.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            base_url: None,
            api_key: api_key.into()
        }
    }

    /// Set the syncthing URL to something different than `http://localhost:8384/rest`.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }


    /// Returns a `Client` that uses this `ClientBuilder` configuration.
    ///
    /// # Errors
    ///
    /// This method fails if the header cannot be created or the HTTP client
    /// cannot be initialized.
    #[must_use]
    pub fn build(self) -> Result<Client> {
        let base_url = self.base_url
            .unwrap_or_else(|| ADDR.to_string());

        let mut headers = header::HeaderMap::new();
        let mut api_key_header = header::HeaderValue::from_str(&self.api_key)?;
        api_key_header.set_sensitive(true);
        headers.insert("X-API-KEY", api_key_header);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Client {
            client,
            base_url
        })
    }
}

/// Abstraction to interact with the Syncthing API.
///
/// The Client has various configuration values to tweak, such as the
/// URL which is set to `localhost:8384/rest` by default. To configure a `Client`,
/// use `Client::builder()`.
#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
    base_url: String,
}

impl Client {
    /// Creates a new HTTP client, with which the syncthing API can be used.
    ///
    /// # Panics
    ///
    /// This method panics if the client cannot be initialized.
    ///
    /// Use `Client::builder()` if you wish to handle the failure as an `Error`
    /// instead of panicking.
    #[must_use]
    pub fn new(api_key: &str) -> Self {
        ClientBuilder::new(api_key).build().expect("Client::new()")
    }

    /// Creates a `ClientBuilder` to configure a `Client`.
    /// This is the same as `ClientBuilder::new()`
    ///
    /// The API can either be generated in the GUI of Syncthing or set
    /// in the configuration file under `configuration/gui/apikey`.
    pub fn builder(api_key: impl Into<String>) -> ClientBuilder {
        ClientBuilder::new(api_key)
    }

    /// Returns `()` if the syncthing API can be reached.
    #[must_use]
    pub async fn ping(&self) -> Result<()> {
        self.client
            .get(format!("{}/system/ping", self.base_url))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Only returns if an error is encountered.
    /// Transmits every new [event](crate::types::events::Event) over `tx`.
    /// If `skip_old`, all events before the call to this function do not
    /// result in a transmission.
    #[must_use]
    pub async fn get_events(&self, tx: Sender<Event>, mut skip_old: bool) -> Result<()> {
        let mut current_id = 0;
        loop {
            let events: Vec<Event> = self
                .client
                .get(format!("{}/events?since={}", self.base_url, current_id))
                .send()
                .await?
                .error_for_status()?
                .json()
                .await?;

            for event in events {
                current_id = event.id;
                if !skip_old {
                    tx.send(event).await?;
                }
            }
            skip_old = false;
        }
    }

    /// Returns the entire [`Configuration`](crate::types::config::Configuration)
    ///
    /// # Errors
    ///
    /// This method fails if the API cannot be reached, the server
    /// answers with an error code or the JSON cannot be parsed.
    #[must_use]
    pub async fn get_configuration(&self) -> Result<Configuration> {
        Ok(self
            .client
            .get(format!("{}/config", ADDR))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}
