---
title: "Configuration"
description: "Any config value can be overridden via env vars: APP_APPLICATION__PORT=5001 sets settings.application.port. This is critical for injecting secrets at runtime without touching version control."
order: 4
---

## Hierarchical Config with the \`config\` Crate

**`configuration/base.yaml`**

```yaml
application:
  port: 8000
  host: "127.0.0.1"
  base_url: "http://127.0.0.1"

database:
  host: "127.0.0.1"
  port: 5432
  username: "postgres"
  password: "password"
  database_name: "newsletter"
  require_ssl: false

email_client:
  base_url: "localhost"
  sender_email: "test@example.com"
  authorization_token: "dev-token"
  timeout_milliseconds: 10000

redis_uri: "redis://127.0.0.1:6379"
```

**`configuration/production.yaml`**

```yaml
application:
  host: "0.0.0.0"
  base_url: "https://api.mynewsletter.com"

database:
  require_ssl: true
```

## Settings Struct

**`src/configuration.rs`**

```rust
use secrecy::{Secret, ExposeSecret};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub email_client: EmailClientSettings,
    pub redis_uri: Secret<String>,
}

#[derive(Deserialize, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
    pub hmac_secret: Secret<String>,
}

#[derive(Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir()
        .expect("Failed to determine current directory");
    let config_dir = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let settings = config::Config::builder()
        .add_source(config::File::from(config_dir.join("base")).required(true))
        .add_source(config::File::from(config_dir.join(environment.as_str())).required(true))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__")
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!("Unknown environment: {other}")),
        }
    }
}
```

  
> [!IMPORTANT]  
> Any config value can be overridden via env vars: APP_APPLICATION__PORT=5001 sets settings.application.port. This is critical for injecting secrets at runtime without touching version control.  

  
> [!IMPORTANT]  
> In a true highly-available environment, restarting the server to change a configuration value (like a log level or a feature flag) is unacceptable. We use the notify crate to watch the configuration file for changes on disk. When a change is detected, we parse it and broadcast the new config globally using tokio::sync::watch. For secrets, instead of \`.env\` files in production, we integrate with HashiCorp Vault or AWS Secrets Manager. Our application fetches a short-lived, dynamically generated database credential at startup and automatically rotates it upon expiration.