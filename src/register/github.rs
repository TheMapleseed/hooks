//! Create GitHub repository webhooks via the GitHub API.

use std::time::Duration;

use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::Deserialize;
use serde_json::json;

use crate::common::Webhook;
use crate::error::{Error, Result};
use crate::provider::Provider;
use crate::secret::Secret;

use super::{RegisteredEndpoint, Registrar};

/// GitHub repository webhook registrar.
#[derive(Debug, Clone)]
pub struct GitHubRegistrar {
    token: Secret,
    owner: String,
    repo: String,
    http: Client,
    api_base: String,
}

impl GitHubRegistrar {
    /// Create a registrar for `owner/repo` using a PAT or GitHub App installation token.
    pub fn new(token: Secret, owner: impl Into<String>, repo: impl Into<String>) -> Self {
        let http = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("reqwest client");
        Self {
            token,
            owner: owner.into(),
            repo: repo.into(),
            http,
            api_base: "https://api.github.com".to_owned(),
        }
    }

    /// Override API base (GHES / tests).
    #[must_use]
    pub fn api_base(mut self, base: impl Into<String>) -> Self {
        self.api_base = base.into();
        self
    }
}

#[derive(Debug, Deserialize)]
struct GitHubHook {
    id: u64,
    #[serde(default)]
    active: bool,
    config: GitHubHookConfig,
    events: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct GitHubHookConfig {
    url: String,
    #[serde(default)]
    content_type: Option<String>,
}

/// Convenience alias for the GitHub registration result.
pub type GitHubRegistration = RegisteredEndpoint;

impl Registrar for GitHubRegistrar {
    fn register(&self, webhook: &Webhook) -> Result<RegisteredEndpoint> {
        if webhook.provider_kind() != Provider::GitHub {
            return Err(Error::RegisterProviderMismatch {
                expected: Provider::GitHub.as_str(),
                actual: webhook.provider(),
            });
        }

        let secret = match webhook.secret_env() {
            Some(name) => Secret::from_env(name)?,
            None => {
                return Err(Error::WebhookMissingField {
                    provider: Provider::GitHub.as_str(),
                    field: "secret_env",
                });
            }
        };

        let body = json!({
            "name": "web",
            "active": true,
            "events": webhook.events(),
            "config": {
                "url": webhook.url(),
                "content_type": "json",
                "secret": secret.expose(),
                "insecure_ssl": "0",
            }
        });

        let url = format!(
            "{}/repos/{}/{}/hooks",
            self.api_base.trim_end_matches('/'),
            self.owner,
            self.repo
        );

        let response = self
            .http
            .post(url)
            .header(ACCEPT, "application/vnd.github+json")
            .header(USER_AGENT, concat!("hooks-rs/", env!("CARGO_PKG_VERSION")))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.token.expose()),
            )
            .header("X-GitHub-Api-Version", "2022-11-28")
            .json(&body)
            .send()
            .map_err(|err| Error::RegisterFailed {
                provider: Provider::GitHub.as_str(),
                reason: err.to_string(),
            })?;

        let status = response.status();
        let text = response.text().unwrap_or_default();
        if !status.is_success() {
            return Err(Error::RegisterFailed {
                provider: Provider::GitHub.as_str(),
                reason: format!("HTTP {status}: {}", truncate(&text, 512)),
            });
        }

        let created: GitHubHook = serde_json::from_str(&text).map_err(|err| {
            Error::RegisterFailed {
                provider: Provider::GitHub.as_str(),
                reason: format!("decode response: {err}"),
            }
        })?;

        Ok(RegisteredEndpoint {
            provider: Provider::GitHub.as_str(),
            id: Some(created.id.to_string()),
            url: created.config.url,
            events: created.events,
            signing_secret: None,
            details: vec![
                ("active".to_owned(), created.active.to_string()),
                (
                    "content_type".to_owned(),
                    created
                        .config
                        .content_type
                        .unwrap_or_else(|| "json".to_owned()),
                ),
                ("repository".to_owned(), format!("{}/{}", self.owner, self.repo)),
            ],
        })
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_owned()
    } else {
        format!("{}…", &s[..max])
    }
}
