---
title: "Testing Strategies"
description: "Example-based tests are fragile. We introduce Property-Based Testing using proptest. Instead of testing with \\"test@example.com\\", proptest generates thousands of edge-case strings (UTF-8 anomalies, null bytes) to ensure our validation logic never panics."
order: 10
---

## Black-Box Integration Testing

**`tests/api/helpers.rs`**

```rust
use once_cell::sync::Lazy;
use sqlx::{PgPool, Connection, Executor};
use uuid::Uuid;
use wiremock::MockServer;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub db_pool: PgPool,
    pub email_server: MockServer,
    pub api_client: reqwest::Client,
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter = "info".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber("test".into(), default_filter, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber("test".into(), default_filter, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let email_server = MockServer::start().await;

    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();
    configuration.application.port = 0;
    configuration.email_client.base_url = email_server.uri();

    let connection_pool = configure_database(&configuration.database).await;

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application");

    let port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    TestApp {
        address: format!("http://127.0.0.1:{port}"),
        port,
        db_pool: connection_pool,
        email_server,
        api_client: client,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    pool
}
```

## Test Example

**`tests/api/subscriptions.rs`**

```rust
use crate::helpers::spawn_app;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula@example.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    let response = app.post_subscriptions(body.into()).await;

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "ursula@example.com");
    assert_eq!(saved.name, "le guin");
}
```

## Property-Based Testing

**`src/domain/subscriber_email.rs`**

```rust
#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(email: ValidEmailFixture) -> bool {
        SubscriberEmail::parse(email.0).is_ok()
    }
}
```

  
> [!IMPORTANT]  
> Example-based tests are fragile. We introduce Property-Based Testing using proptest. Instead of testing with "test@example.com", proptest generates thousands of edge-case strings (UTF-8 anomalies, null bytes) to ensure our validation logic never panics. For mission-critical parsers, we use cargo-fuzz (libFuzzer) to continuously bombard our code with mutated inputs. For concurrent data structures, we use loom, which permutes all possible thread interleavings to mathematically prove the absence of race conditions.