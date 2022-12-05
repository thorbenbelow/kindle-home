#[derive(serde::Deserialize, Clone, Debug)]
pub struct HueClientSettings {
    pub username: String,
    pub base_url: String,
}
pub struct HueClient {
    http_client: reqwest::Client,
    base_url: String,
    username: String,
}

pub enum LightState {
    On,
    Off,
}

impl Into<bool> for LightState {
    fn into(self) -> bool {
        match self {
            LightState::On => true,
            LightState::Off => false,
        }
    }
}

impl From<bool> for LightState {
    fn from(value: bool) -> Self {
        if value {
            LightState::On
        } else {
            LightState::Off
        }
    }
}

impl HueClient {
    pub fn new(settings: HueClientSettings) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            base_url: settings.base_url,
            username: settings.username,
        }
    }

    pub async fn set_light(&self, id: u32, state: LightState) -> Result<(), String> {
        let state: bool = state.into();
        let response = self.http_client
            .put(format!(
                "http://{}/{}/lights/{}/state",
                self.base_url, self.username, id
            ))
            .body(format!("{{\"on\": {}}}", state))
            .send()
            .await;
        if response.is_ok() {
            Ok(())
        } else {
            Err("Failed to set light state".to_owned())
        }
    }
}
