//! Secret handling for signing and provider API credentials.
//!
//! Secrets are never serialized into emitted webhook configs. Prefer loading
//! from the environment via [`Secret::from_env`] or [`crate::Webhook::secret_env`].

use std::env;
use std::fmt;

use secrecy::{ExposeSecret, SecretString};
use zeroize::Zeroize;

use crate::error::{Error, Result};

/// An opaque secret value (signing key or API token).
///
/// Debug/Display redact the contents. Drop zeroes the underlying bytes via
/// [`secrecy`]/[`SecretString`].
#[derive(Clone)]
pub struct Secret {
    inner: SecretString,
}

impl Secret {
    /// Wrap an owned secret string.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        let mut value = value.into();
        let secret = SecretString::from(value.clone());
        value.zeroize();
        Self { inner: secret }
    }

    /// Load a secret from an environment variable.
    pub fn from_env(name: impl AsRef<str>) -> Result<Self> {
        let name = name.as_ref();
        let value = env::var(name).map_err(|_| Error::MissingSecretEnv {
            name: name.to_owned(),
        })?;
        if value.trim().is_empty() {
            return Err(Error::MissingSecretEnv {
                name: name.to_owned(),
            });
        }
        Ok(Self::new(value))
    }

    /// Borrow the secret bytes for cryptographic use only.
    #[must_use]
    pub fn expose(&self) -> &str {
        self.inner.expose_secret()
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Secret([REDACTED])")
    }
}
