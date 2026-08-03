---
title: "Validation & Type-Driven Design"
description: "Instead of scattering is_valid() checks everywhere, create types that make invalid states unrepresentable."
order: 8
---

## Parse, Don't Validate

Instead of scattering `is_valid()` checks everywhere, create types that **make invalid states unrepresentable**.

**`src/domain/subscriber_email.rs`**

```rust
use garde::Validate;

#[derive(Debug, Clone)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if garde::Validate::validate(&EmailCheck { email: &s }, &()).is_ok() {
            Ok(Self(s))
        } else {
            Err(format!("{s} is not a valid subscriber email."))
        }
    }
}

#[derive(Validate)]
struct EmailCheck<'a> {
    #[garde(email)]
    email: &'a str,
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SubscriberEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
```

**`src/domain/subscriber_name.rs`**

```rust
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let has_forbidden = s.chars().any(|c| forbidden.contains(&c));

        if is_empty || is_too_long || has_forbidden {
            Err(format!("{s} is not a valid subscriber name."))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
```

## Composing Validated Types

**`src/domain/new_subscriber.rs`**

```rust
use crate::domain::{SubscriberEmail, SubscriberName};

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}

impl TryFrom<SubscribeForm> for NewSubscriber {
    type Error = String;

    fn try_from(form: SubscribeForm) -> Result<Self, Self::Error> {
        let email = SubscriberEmail::parse(form.email)?;
        let name = SubscriberName::parse(form.name)?;
        Ok(Self { email, name })
    }
}
```

  
> [!IMPORTANT]  
> Once you have a SubscriberEmail, it's impossible to pass an invalid email to insert_subscriber. The compiler enforces your domain invariants at every call site.  

  
> [!IMPORTANT]  
> Instead of relying on runtime validation checks scattered throughout the code, we encode state transitions into the type system—making invalid states unrepresentable at compile time. **`src/domain/subscriber.rs`** pub struct UnconfirmedSubscriber { id: Uuid } pub struct ConfirmedSubscriber { id: Uuid } impl UnconfirmedSubscriber { pub fn confirm(self) -> ConfirmedSubscriber { // Consumes \`self\`, returns new type ConfirmedSubscriber { id: self.id } } } A function requiring a ConfirmedSubscriber physically cannot be passed an unconfirmed one. The compiler enforces our business logic.