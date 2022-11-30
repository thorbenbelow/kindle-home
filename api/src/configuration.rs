use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub app: AppSettings,
    pub hue: HueSettings,
}

#[derive(serde::Deserialize)]
pub struct HueSettings {
    pub username: String,
}

#[derive(serde::Deserialize)]
pub struct AppSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

impl AppSettings {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Environment {
    Dev,
    Prod,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Dev => "dev",
            Environment::Prod => "prod",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "dev" => Ok(Environment::Dev),
            "prod" => Ok(Environment::Prod),
            other => Err(format!(
                "{} is not a supported environment! Use 'dev' or 'prod' instead.",
                other
            )),
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "dev".to_string())
            .try_into()
            .expect("Failed to parse APP_ENVIRONMENT.");

        let base_path = std::env::current_dir().expect("Failed to determine the current directory");
        let configuration_directory = base_path.join("config");

        if environment == Environment::Dev {
            dotenv::dotenv().ok();
        }

        let settings = config::Config::builder()
            .add_source(
                config::File::from(configuration_directory.join(environment.as_str()))
                    .required(true),
            )
            .add_source(
                config::Environment::with_prefix("APP")
                    .prefix_separator("__")
                    .separator("_"),
            )
            .build()?;

        settings.try_deserialize::<Settings>()
    }
}
