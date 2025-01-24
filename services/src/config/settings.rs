use secrecy::{ExposeSecret as _, SecretString};
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[doc = "Settings for the app"]
#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub application: AppSettings,
    pub jwt: JwtSettings,
    pub db: DatabaseSettings,
}

#[doc = "Settings for application (host, port, cors)"]
#[derive(serde::Deserialize, Clone)]
pub struct AppSettings {
    pub host: String,
    pub port: u16,
    pub cors_location: String,
    pub api_prefix: String,
}
#[doc = "Settings for JWT security"]
#[derive(serde::Deserialize, Clone)]
pub struct JwtSettings {
    pub secret: SecretString,
    pub expires_in: String,
    pub max_age: i64,
    pub cookie_domain: String,
    pub cookie_name: String,
}
#[doc = "Configuration for the database"]
#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: SecretString,
    pub password: SecretString,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn read_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new("config.yaml", config::FileFormat::Yaml))
        .build()?;
    settings.try_deserialize()
}

impl AppSettings {
    #[doc = "Retrieve the URL address of the application."]
    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl DatabaseSettings {
    #[doc = "Retrieve the PgConnectOptions for the database"]
    pub fn get_options(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .database(&self.database_name)
            .host(&self.host)
            .username(self.username.expose_secret())
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(PgSslMode::Prefer)
    }
}
