pub mod models;

#[cfg(test)]
mod tests {
    use super::models::*;
    use uuid::Uuid;

    #[test]
    fn test_models_json_serialization() {
        let subscriber = NewsletterSubscriber {
            id: Uuid::new_v4(),
            email: "engineer@example.com".to_string(),
            is_active: true,
            subscribed_at: chrono::Utc::now(),
        };

        let json = serde_json::to_string(&subscriber).expect("Serialization failed");
        let deserialized: NewsletterSubscriber = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(subscriber.email, deserialized.email);
        assert_eq!(subscriber.is_active, deserialized.is_active);

        let playground = PlaygroundResponse {
            success: true,
            stdout: "42\n".to_string(),
            stderr: String::new(),
        };
        let p_json = serde_json::to_string(&playground).expect("Playground serialize failed");
        assert!(p_json.contains("42"));
    }
}
