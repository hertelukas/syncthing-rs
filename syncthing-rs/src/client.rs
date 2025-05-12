use crate::{
    error::{Error, Result},
    types::{
        cluster::{PendingDevices, PendingFolders},
        config::{
            Configuration, DeviceConfiguration, FolderConfiguration, NewDeviceConfiguration,
            NewFolderConfiguration,
        },
        events::Event,
    },
};
use reqwest::{StatusCode, header};
use tokio::sync::broadcast::Sender;

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
            api_key: api_key.into(),
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
    pub fn build(self) -> Result<Client> {
        let base_url = self.base_url.unwrap_or_else(|| ADDR.to_string());

        let mut headers = header::HeaderMap::new();
        let mut api_key_header = header::HeaderValue::from_str(&self.api_key)?;
        api_key_header.set_sensitive(true);
        headers.insert("X-API-KEY", api_key_header);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Client { client, base_url })
    }
}

/// Abstraction to interact with the Syncthing API.
///
/// The Client has various configuration values to tweak, such as the
/// URL which is set to `localhost:8384/rest` by default. To configure a `Client`,
/// use `Client::builder()`.
#[derive(Clone, Debug)]
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
    ///
    /// Use [`health`](crate::client::Client::health) to do the same
    /// without the need of a valid `api_key`.
    pub async fn ping(&self) -> Result<()> {
        log::debug!("GET /system/ping");
        self.client
            .get(format!("{}/system/ping", self.base_url))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Returns `()` if the syncthing API can be reached.
    ///
    /// Use [`ping`](crate::client::Client::ping) to do the same
    /// but with the requirement of a valid `api_key`.
    pub async fn health(&self) -> Result<()> {
        log::debug!("GET /noauth/health");
        self.client
            .get(format!("{}/noauth/health", self.base_url))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Returns the ID of the current device. This endpoint
    /// does not require a valid `api_key`.
    pub async fn get_id(&self) -> Result<String> {
        log::debug!("GET /noauth/health");
        Ok(self
            .client
            .get(format!("{}/noauth/health", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .headers()
            .get("X-Syncthing-Id")
            .ok_or(Error::HeaderDeviceIDError)?
            .to_str()
            .map_err(|_| Error::HeaderParseError)?
            .to_string())
    }

    /// Only returns if an error is encountered.
    /// Transmits every new [event](crate::types::events::Event) over `tx`.
    /// If `skip_old`, all events before the call to this function do not
    /// result in a transmission.
    pub async fn get_events(&self, tx: Sender<Event>, mut skip_old: bool) -> Result<()> {
        let mut current_id = 0;
        loop {
            log::debug!("GET /events");
            let events: Vec<Event> = self
                .client
                .get(format!("{}/events?since={}", self.base_url, current_id))
                .send()
                .await?
                .error_for_status()?
                .json()
                .await?;

            log::debug!("received {} new events", events.len());
            for event in events {
                current_id = event.id;
                if !skip_old {
                    tx.send(event)?;
                }
            }
            log::debug!("current event id is {current_id}");
            skip_old = false;
        }
    }

    /// Returns the entire [`Configuration`]
    ///
    /// # Errors
    ///
    /// This method fails if the API cannot be reached, the server
    /// answers with an error code or the JSON cannot be parsed.
    pub async fn get_configuration(&self) -> Result<Configuration> {
        log::debug!("GET /config");
        Ok(self
            .client
            .get(format!("{}/config", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Posts a folder. If the folder already exists, it is replaced,
    /// otherwise a new one is added.
    ///
    /// Use [`add_folder`](crate::client::Client::add_folder) if the operation
    /// should fail if a folder with the same ID already exists.
    pub async fn post_folder(&self, folder: impl Into<NewFolderConfiguration>) -> Result<()> {
        let folder = folder.into();
        log::debug!("POST /config/folders {:?}", folder);
        self.client
            .post(format!("{}/config/folders", self.base_url))
            .json(&folder)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Adds a new folder. If the folder already exists, a
    /// [`DuplicateFolderError`](crate::error::Error::DuplicateFolderError) is returned.
    /// This requires an additional check against the API.
    ///
    /// Use [`post_folder`](crate::client::Client::post_folder) if the operation
    /// should blindly set the folder.
    pub async fn add_folder(&self, folder: impl Into<NewFolderConfiguration>) -> Result<()> {
        let folder = folder.into();
        match self.get_folder(folder.get_id()).await {
            Ok(_) => return Err(Error::DuplicateFolderError),
            Err(Error::UnknownFolderError) => (),
            Err(e) => return Err(e),
        }
        self.post_folder(folder).await
    }

    /// Gets the configuration for the folder with the ID `folder_id`. Explicitly
    /// returns a [`UnknownFolderError`](crate::error::Error::UnknownFolderError)
    /// if no folder with `folder_id` exists.
    pub async fn get_folder(&self, folder_id: &str) -> Result<FolderConfiguration> {
        log::debug!("GET /config/folders/{}", folder_id);
        let response = self
            .client
            .get(format!("{}/config/folders/{}", self.base_url, folder_id))
            .send()
            .await?;

        if response.status() == StatusCode::NOT_FOUND {
            // TODO check that really the folder ID is causing that
            Err(Error::UnknownFolderError)
        } else {
            Ok(response.error_for_status()?.json().await?)
        }
    }

    /// Posts a device. If the device already exists, it is replaced,
    /// otherwise a new one is added.
    ///
    /// Use [`add_device`](crate::client::Client::add_device) if the operation
    /// should fail if a device with the same ID already exists.
    pub async fn post_device(&self, device: impl Into<NewDeviceConfiguration>) -> Result<()> {
        let device = device.into();
        log::debug!("POST /config/devices {:?}", device);
        self.client
            .post(format!("{}/config/devices", self.base_url))
            .json(&device)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Adds a new device. If the device already exists, a
    /// [`DuplicateDeviceError`](crate::error::Error::DuplicateDeviceError) is returned.
    /// This requires an additional check against the API.
    ///
    /// Use [`post_device`](crate::client::Client::post_device) if the operation
    /// should blindly set the device.
    pub async fn add_device(&self, device: impl Into<NewDeviceConfiguration>) -> Result<()> {
        let device = device.into();
        match self.get_device(device.get_device_id()).await {
            Ok(_) => return Err(Error::DuplicateDeviceError),
            Err(Error::UnknownDeviceError) => (),
            Err(e) => return Err(e),
        }
        self.post_device(device).await
    }

    /// Gets the configuration for the device with the ID `device_id`.
    pub async fn get_device(&self, device_id: &str) -> Result<DeviceConfiguration> {
        log::debug!("GET /config/devices/{}", device_id);
        let response = self
            .client
            .get(format!("{}/config/devices/{}", self.base_url, device_id))
            .send()
            .await?;

        if response.status() == StatusCode::NOT_FOUND {
            // TODO check that really the device ID is causing that
            Err(Error::UnknownDeviceError)
        } else {
            Ok(response.error_for_status()?.json().await?)
        }
    }

    /// Gets a list of all pending remote devices which have tried to connect but
    /// are not in our configuration yet.
    pub async fn get_pending_devices(&self) -> Result<PendingDevices> {
        log::debug!("GET /cluster/pending/devices");
        Ok(self
            .client
            .get(format!("{}/cluster/pending/devices", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Gets all folders which remote devices have offered to us, but are not yet shared
    /// from our instance to them or are not present on our instance.
    pub async fn get_pending_folders(&self) -> Result<PendingFolders> {
        log::debug!("GET /cluster/pending/folders");
        Ok(self
            .client
            .get(format!("{}/cluster/pending/folders", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Remove record about pending remote device with ID `device_id` which tried to connect.
    ///
    /// This is not permanent, use `ignore_device` instead.
    pub async fn delete_pending_device(&self, device_id: &str) -> Result<()> {
        log::debug!("DELETE /cluster/pending/devices?device={device_id}");
        self.client
            .delete(format!(
                "{}/cluster/pending/devices?device={}",
                self.base_url, device_id
            ))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Remove record about pending remote folder with ID `folder_id`. An optional `device_id`
    /// can be passed as argument to only remove the pending remote from that device, otherwise
    /// the folder will be removed as pending for all devices.
    ///
    /// This is not permanent, use `ignore_folder` instead.
    pub async fn delete_pending_folder(
        &self,
        folder_id: &str,
        device_id: Option<&str>,
    ) -> Result<()> {
        let device_str = match device_id {
            Some(device_id) => format!("?device={}", device_id),
            None => String::new(),
        };
        log::debug!("DELETE /clusterpending/folders?folder={folder_id}{device_str}");
        self.client
            .delete(format!(
                "{}/cluster/pending/folders?folder={}{}",
                self.base_url, folder_id, device_str
            ))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Returns a template device configuration with all default values,
    /// which only requires a unique device ID to be instantiated.
    pub async fn get_default_device(&self) -> Result<DeviceConfiguration> {
        log::debug!("GET /config/defaults/device");
        Ok(self
            .client
            .get(format!("{}/config/defaults/device", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Returns a template folder configuration with all default values,
    /// which only requires a unique folder ID to be instantiated.
    pub async fn get_default_folder(&self) -> Result<FolderConfiguration> {
        log::debug!("GET /config/defaults/folder");
        Ok(self
            .client
            .get(format!("{}/config/defaults/folder", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::events::EventType;

    use super::*;

    use httpmock::prelude::*;
    use testcontainers::{
        ContainerAsync, GenericImage, ImageExt,
        core::{ContainerPort::Tcp, WaitFor},
        runners::AsyncRunner,
    };
    use tokio::sync::broadcast;

    use rstest::*;

    // Example device id from the docs
    const DEVICE_ID: &str = "MFZWI3D-BONSGYC-YLTMRWG-C43ENR5-QXGZDMM-FZWI3DP-BONSGYY-LTMRWAD";

    #[fixture]
    async fn syncthing_setup() -> (ContainerAsync<GenericImage>, Client) {
        let api_key = "foobar";
        let container = GenericImage::new("syncthing/syncthing", "latest")
            .with_exposed_port(Tcp(8384))
            .with_wait_for(WaitFor::message_on_stdout("GUI and API listening on "))
            .with_env_var("STGUIAPIKEY", api_key)
            .start()
            .await
            .expect("failed to start syncthing container");

        let host = container
            .get_host()
            .await
            .expect("could not get syncthing host");
        let port = container
            .get_host_port_ipv4(8384)
            .await
            .expect("could not get syncthing port");

        let url = format!("http://{host}:{port}/rest");

        let client = ClientBuilder::new(api_key).base_url(url).build().unwrap();

        (container, client)
    }

    #[test]
    fn test_new() {
        let client = Client::new("foo");

        assert_eq!(client.base_url, "http://localhost:8384/rest");
    }

    /// Simple ping to a running server should just return Ok(())
    #[tokio::test]
    async fn test_ping() {
        let server = MockServer::start();

        let ping_mock = server.mock(|when, then| {
            when.method(GET).path("/system/ping");
            then.status(200)
                .header("content-type", "application/json")
                .body(r#"{"ping": "pong"}"#);
        });

        let client = ClientBuilder::new("")
            .base_url(server.base_url())
            .build()
            .unwrap();

        let result = client.ping().await;
        ping_mock.assert();

        assert!(result.is_ok());
    }

    /// Simple test ensuring that a single event in the past is correctly
    /// transmitted.
    #[tokio::test]
    async fn test_single_event() {
        let server = MockServer::start();

        let event_mock = server.mock(|when, then| {
            when.method(GET).path("/events");
            then.status(200)
                .header("content-type", "application/json")
                .body(
                    r#"
[
  {
    "id": 1,
    "globalID": 1,
    "time": "2025-05-07T17:05:44.514050967+02:00",
    "type": "Starting",
    "data": {
      "home": "/home/user/.config/syncthing",
      "myID": "XXXXXXX-XXXXXXX-XXXXXXX-XXXXXXX-XXXXXXX-XXXXXXX-XXXXXXX-XXXXXXX"
    }
  }
]
"#,
                );
        });

        let client = ClientBuilder::new("")
            .base_url(server.base_url())
            .build()
            .unwrap();

        let (tx, mut rx) = broadcast::channel(1);

        // Start transmitting events on a separate thread
        tokio::spawn(async move {
            let result = client.get_events(tx, false).await;
            unreachable!("get_events should not have returned: {:?}", result);
        });

        let event = rx.recv().await;
        assert!(event_mock.hits() > 0);
        assert!(event.is_ok());
        assert_eq!(event.unwrap().ty, EventType::Starting {})
    }

    #[tokio::test]
    async fn container_test_health() {
        // Create container by hand, so we don't know the API key. This is okay
        // as the health endpoint should work anyway
        let container = GenericImage::new("syncthing/syncthing", "latest")
            .with_exposed_port(Tcp(8384))
            .with_wait_for(WaitFor::message_on_stdout("GUI and API listening on "))
            .start()
            .await
            .expect("failed to start syncthing container");

        let host = container
            .get_host()
            .await
            .expect("could not get syncthing host");
        let port = container
            .get_host_port_ipv4(8384)
            .await
            .expect("could not get syncthing port");

        let url = format!("http://{host}:{port}/rest");

        let client = ClientBuilder::new("idk").base_url(url).build().unwrap();

        client.health().await.unwrap();
    }

    #[tokio::test]
    async fn container_test_id() {
        // Create container by hand, so we don't know the API key. This is okay
        // as the id endpoint should work anyway
        let container = GenericImage::new("syncthing/syncthing", "latest")
            .with_exposed_port(Tcp(8384))
            .with_wait_for(WaitFor::message_on_stdout("GUI and API listening on "))
            .start()
            .await
            .expect("failed to start syncthing container");

        let host = container
            .get_host()
            .await
            .expect("could not get syncthing host");
        let port = container
            .get_host_port_ipv4(8384)
            .await
            .expect("could not get syncthing port");

        let url = format!("http://{host}:{port}/rest");

        let client = ClientBuilder::new("idk").base_url(url).build().unwrap();

        client.get_id().await.unwrap();
    }

    #[rstest]
    #[tokio::test]
    async fn container_test_ping(
        #[future] syncthing_setup: (ContainerAsync<GenericImage>, Client),
    ) {
        let (_container, client) = syncthing_setup.await;

        client.ping().await.unwrap();
    }

    #[rstest]
    #[tokio::test]
    async fn container_test_get_config(
        #[future] syncthing_setup: (ContainerAsync<GenericImage>, Client),
    ) {
        let (_container, client) = syncthing_setup.await;

        client
            .get_configuration()
            .await
            .expect("could not get config");
    }

    #[rstest]
    #[tokio::test]
    async fn container_test_post_folder(
        #[future] syncthing_setup: (ContainerAsync<GenericImage>, Client),
    ) {
        let (_container, client) = syncthing_setup.await;
        let folder_id = "this-is-a-new-folder";
        let path = "/tmp";

        let folder = NewFolderConfiguration::new(folder_id.to_string(), path.to_string());

        client
            .post_folder(folder)
            .await
            .expect("could not post folder");

        let api_folder = client
            .get_folder(folder_id)
            .await
            .expect("could not get folder");

        assert_eq!(&api_folder.id, folder_id);
        assert_eq!(&api_folder.path, path);
    }

    #[rstest]
    #[tokio::test]
    async fn container_test_add_folder(
        #[future] syncthing_setup: (ContainerAsync<GenericImage>, Client),
    ) {
        let (_container, client) = syncthing_setup.await;
        let folder_id = "this-is-a-new-folder";
        let path = "/tmp";

        let folder = NewFolderConfiguration::new(folder_id.to_string(), path.to_string());

        client
            .add_folder(folder)
            .await
            .expect("could not add folder");

        let api_folder = client
            .get_folder(folder_id)
            .await
            .expect("could not get folder");

        assert_eq!(&api_folder.id, folder_id);
        assert_eq!(&api_folder.path, path);
    }

    #[rstest]
    #[tokio::test]
    #[should_panic(expected = "DuplicateFolderError")]
    async fn container_test_add_folder_twice_panic(
        #[future] syncthing_setup: (ContainerAsync<GenericImage>, Client),
    ) {
        let (_container, client) = syncthing_setup.await;
        let folder_id = "this-is-a-new-folder";
        let path = "/tmp";

        let folder = NewFolderConfiguration::new(folder_id.to_string(), path.to_string());

        client
            .add_folder(folder)
            .await
            .expect("could not add folder");

        // "Accidentally" overwrite our folder
        let duplicate_path = "/usr";
        let duplicate_folder =
            NewFolderConfiguration::new(folder_id.to_string(), duplicate_path.to_string());

        client
            .add_folder(duplicate_folder)
            .await
            .expect("could not add folder")
    }

    #[rstest]
    #[tokio::test]
    async fn container_test_post_folder_twice(
        #[future] syncthing_setup: (ContainerAsync<GenericImage>, Client),
    ) {
        let (_container, client) = syncthing_setup.await;
        let folder_id = "this-is-a-new-folder";
        let path = "/tmp";

        let folder = NewFolderConfiguration::new(folder_id.to_string(), path.to_string());

        client
            .add_folder(folder)
            .await
            .expect("could not add folder");

        // "Accidentally" overwrite our folder
        let duplicate_path = "/usr";
        let duplicate_folder =
            NewFolderConfiguration::new(folder_id.to_string(), duplicate_path.to_string());

        client
            .post_folder(duplicate_folder)
            .await
            .expect("could not post folder");

        let api_folder = client
            .get_folder(folder_id)
            .await
            .expect("could not get folder");

        assert_eq!(&api_folder.id, folder_id);
        assert_eq!(&api_folder.path, duplicate_path);
    }

    #[rstest]
    #[tokio::test]
    async fn container_test_post_device(
        #[future] syncthing_setup: (ContainerAsync<GenericImage>, Client),
    ) {
        let (_container, client) = syncthing_setup.await;

        let device = NewDeviceConfiguration::new(DEVICE_ID.to_string());

        client
            .post_device(device)
            .await
            .expect("could not post device");

        let api_device = client
            .get_device(DEVICE_ID)
            .await
            .expect("could not get device");

        assert_eq!(&api_device.device_id, DEVICE_ID);
    }

    #[rstest]
    #[tokio::test]
    async fn container_test_add_device(
        #[future] syncthing_setup: (ContainerAsync<GenericImage>, Client),
    ) {
        let (_container, client) = syncthing_setup.await;

        let device = NewDeviceConfiguration::new(DEVICE_ID.to_string());

        client
            .add_device(device)
            .await
            .expect("could not add device");

        let api_device = client
            .get_device(DEVICE_ID)
            .await
            .expect("could not get device");

        assert_eq!(&api_device.device_id, DEVICE_ID);
    }

    #[rstest]
    #[tokio::test]
    #[should_panic(expected = "DuplicateDeviceError")]
    async fn container_test_add_device_twice_panic(
        #[future] syncthing_setup: (ContainerAsync<GenericImage>, Client),
    ) {
        let (_container, client) = syncthing_setup.await;

        let device = NewDeviceConfiguration::new(DEVICE_ID.to_string());

        client
            .add_device(device)
            .await
            .expect("could not add device");

        // "Accidentally" overwrite our device
        let duplicate_device = NewDeviceConfiguration::new(DEVICE_ID.to_string());

        client
            .add_device(duplicate_device)
            .await
            .expect("could not add device")
    }

    #[rstest]
    #[tokio::test]
    async fn container_test_post_device_twice(
        #[future] syncthing_setup: (ContainerAsync<GenericImage>, Client),
    ) {
        let (_container, client) = syncthing_setup.await;
        let name = "original";

        let device = NewDeviceConfiguration::new(DEVICE_ID.to_string()).name(name.to_string());

        client
            .add_device(device)
            .await
            .expect("could not add device");

        // "Accidentally" overwrite our device
        let duplicate_name = "duplicate";
        let duplicate_device =
            NewDeviceConfiguration::new(DEVICE_ID.to_string()).name(duplicate_name.to_string());

        client
            .post_device(duplicate_device)
            .await
            .expect("could not post device");

        let api_device = client
            .get_device(DEVICE_ID)
            .await
            .expect("could not get device");

        assert_eq!(&api_device.device_id, DEVICE_ID);
        assert_eq!(&api_device.name, duplicate_name);
    }

    #[rstest]
    #[tokio::test]
    async fn container_test_pending_device(
        #[future]
        #[from(syncthing_setup)]
        first: (ContainerAsync<GenericImage>, Client),
        #[future]
        #[from(syncthing_setup)]
        second: (ContainerAsync<GenericImage>, Client),
    ) {
        let (_first_container, first_client) = first.await;
        let (_second_container, second_client) = second.await;

        let first_id = first_client
            .get_id()
            .await
            .expect("could not get id of first container");
        let second_id = second_client
            .get_id()
            .await
            .expect("could not get id of second container");

        // First starts waiting for the event
        let (event_tx, mut event_rx) = broadcast::channel(10);
        let first_client_handle = first_client.clone();
        tokio::spawn(async move {
            first_client_handle
                .get_events(event_tx, true)
                .await
                .unwrap();
        });

        // Add the first device to the second
        second_client
            .add_device(NewDeviceConfiguration::new(first_id))
            .await
            .expect("could not add device");

        // Now wait until we get an added device event on the first container
        loop {
            let event = event_rx.recv().await.unwrap();
            match event.ty {
                EventType::PendingDevicesChanged {
                    added: Some(added), ..
                } => {
                    if !added.is_empty() {
                        break;
                    }
                }
                // Skip other events
                _ => {}
            }
        }

        // Check that this device is the correct one
        let pending = first_client
            .get_pending_devices()
            .await
            .expect("could not get pending devices");
        assert!(pending.devices.contains_key(&second_id));
    }

    #[rstest]
    #[tokio::test]
    async fn container_test_delete_pending_device(
        #[future]
        #[from(syncthing_setup)]
        first: (ContainerAsync<GenericImage>, Client),
        #[future]
        #[from(syncthing_setup)]
        second: (ContainerAsync<GenericImage>, Client),
    ) {
        let (_first_container, first_client) = first.await;
        let (_second_container, second_client) = second.await;

        let first_id = first_client
            .get_id()
            .await
            .expect("could not get id of first container");
        let second_id = second_client
            .get_id()
            .await
            .expect("could not get id of second container");

        // First starts waiting for the event
        let (event_tx, mut event_rx) = broadcast::channel(10);
        let first_client_handle = first_client.clone();
        tokio::spawn(async move {
            first_client_handle
                .get_events(event_tx, true)
                .await
                .unwrap();
        });

        // Add the first device to the second
        second_client
            .add_device(NewDeviceConfiguration::new(first_id))
            .await
            .expect("could not add device");

        // Now wait until we get an added device event on the first container
        loop {
            let event = event_rx.recv().await.unwrap();
            match event.ty {
                EventType::PendingDevicesChanged {
                    added: Some(added), ..
                } => {
                    if !added.is_empty() {
                        break;
                    }
                }
                // Skip other events
                _ => {}
            }
        }

        // Check that this device is the correct one
        let pending = first_client
            .get_pending_devices()
            .await
            .expect("could not get pending devices");
        assert!(pending.devices.contains_key(&second_id));

        first_client
            .delete_pending_device(&second_id)
            .await
            .expect("could not delete pending device");

        // Now wait until we get a removed device event in the first container
        loop {
            let event = event_rx.recv().await.unwrap();
            match event.ty {
                EventType::PendingDevicesChanged {
                    removed: Some(removed),
                    ..
                } => {
                    if !removed.is_empty() {
                        break;
                    }
                }
                // Skip other events
                _ => {}
            }
        }

        // Check that the device is no longer there
        let pending = first_client
            .get_pending_devices()
            .await
            .expect("could not get pending devices");
        assert!(!pending.devices.contains_key(&second_id));
        // There shouldn't be any pending device anymore
        assert_eq!(pending.devices.len(), 0)
    }

    #[rstest]
    #[tokio::test]
    async fn container_test_get_default_device(
        #[future] syncthing_setup: (ContainerAsync<GenericImage>, Client),
    ) {
        let (_container, client) = syncthing_setup.await;

        client
            .get_default_device()
            .await
            .expect("could not get default device");
    }

    #[rstest]
    #[tokio::test]
    async fn container_test_get_default_folder(
        #[future] syncthing_setup: (ContainerAsync<GenericImage>, Client),
    ) {
        let (_container, client) = syncthing_setup.await;

        client
            .get_default_folder()
            .await
            .expect("could not get default folder");
    }
}
