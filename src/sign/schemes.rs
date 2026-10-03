//! Concrete signing implementations.

use std::collections::BTreeMap;

use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use hmac::{Hmac, Mac};
use sha1::Sha1;
use sha2::Sha256;

use crate::error::{Error, Result};
use crate::secret::Secret;

use super::SignRequest;

type HmacSha256 = Hmac<Sha256>;
type HmacSha1 = Hmac<Sha1>;

/// Signed header set ready for an HTTP request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedRequest {
    /// Extra headers to attach (signature / timestamp / id).
    pub headers: BTreeMap<String, String>,
}

fn hmac_sha256(secret: &Secret, message: &[u8]) -> Result<Vec<u8>> {
    let mut mac = HmacSha256::new_from_slice(secret.expose().as_bytes()).map_err(|_| {
        Error::SigningFailed {
            reason: "invalid HMAC key length",
        }
    })?;
    mac.update(message);
    Ok(mac.finalize().into_bytes().to_vec())
}

fn hmac_sha1(secret: &Secret, message: &[u8]) -> Result<Vec<u8>> {
    let mut mac = HmacSha1::new_from_slice(secret.expose().as_bytes()).map_err(|_| {
        Error::SigningFailed {
            reason: "invalid HMAC key length",
        }
    })?;
    mac.update(message);
    Ok(mac.finalize().into_bytes().to_vec())
}

pub(super) fn stripe(request: &SignRequest<'_>) -> Result<SignedRequest> {
    let signed = format!("{}.{}", request.timestamp, String::from_utf8_lossy(request.body));
    let digest = hmac_sha256(request.secret, signed.as_bytes())?;
    let header = format!("t={},v1={}", request.timestamp, hex::encode(digest));
    let mut headers = BTreeMap::new();
    headers.insert("Stripe-Signature".to_owned(), header);
    headers.insert("Content-Type".to_owned(), "application/json".to_owned());
    Ok(SignedRequest { headers })
}

pub(super) fn github_sha256(request: &SignRequest<'_>) -> Result<SignedRequest> {
    let digest = hmac_sha256(request.secret, request.body)?;
    let header_name = request
        .scheme
        .header()
        .unwrap_or("X-Hub-Signature-256")
        .to_owned();
    let mut headers = BTreeMap::new();
    headers.insert(header_name, format!("sha256={}", hex::encode(digest)));
    headers.insert("Content-Type".to_owned(), "application/json".to_owned());
    Ok(SignedRequest { headers })
}

pub(super) fn slack(request: &SignRequest<'_>) -> Result<SignedRequest> {
    let basestring = format!(
        "v0:{}:{}",
        request.timestamp,
        String::from_utf8_lossy(request.body)
    );
    let digest = hmac_sha256(request.secret, basestring.as_bytes())?;
    let mut headers = BTreeMap::new();
    headers.insert(
        "X-Slack-Signature".to_owned(),
        format!("v0={}", hex::encode(digest)),
    );
    headers.insert(
        "X-Slack-Request-Timestamp".to_owned(),
        request.timestamp.to_string(),
    );
    headers.insert("Content-Type".to_owned(), "application/json".to_owned());
    Ok(SignedRequest { headers })
}

pub(super) fn shopify(request: &SignRequest<'_>) -> Result<SignedRequest> {
    let digest = hmac_sha256(request.secret, request.body)?;
    let mut headers = BTreeMap::new();
    headers.insert("X-Shopify-Hmac-Sha256".to_owned(), B64.encode(digest));
    headers.insert("Content-Type".to_owned(), "application/json".to_owned());
    Ok(SignedRequest { headers })
}

pub(super) fn standard_webhooks(request: &SignRequest<'_>) -> Result<SignedRequest> {
    let msg_id = request.message_id.ok_or(Error::SigningFailed {
        reason: "standard webhooks require a message_id",
    })?;
    let secret_bytes = decode_standard_secret(request.secret)?;
    let signed = format!(
        "{msg_id}.{}.{}",
        request.timestamp,
        String::from_utf8_lossy(request.body)
    );
    let mut mac = HmacSha256::new_from_slice(&secret_bytes).map_err(|_| Error::SigningFailed {
        reason: "invalid HMAC key length",
    })?;
    mac.update(signed.as_bytes());
    let sig = B64.encode(mac.finalize().into_bytes());

    let mut headers = BTreeMap::new();
    headers.insert("webhook-id".to_owned(), msg_id.to_owned());
    headers.insert(
        "webhook-timestamp".to_owned(),
        request.timestamp.to_string(),
    );
    headers.insert("webhook-signature".to_owned(), format!("v1,{sig}"));
    headers.insert("Content-Type".to_owned(), "application/json".to_owned());
    Ok(SignedRequest { headers })
}

fn decode_standard_secret(secret: &Secret) -> Result<Vec<u8>> {
    let raw = secret.expose();
    let b64 = raw.strip_prefix("whsec_").unwrap_or(raw);
    B64.decode(b64).map_err(|_| Error::SigningFailed {
        reason: "standard webhook secret must be whsec_ + base64 (or raw base64)",
    })
}

pub(super) fn hex_hmac_sha256_body(request: &SignRequest<'_>) -> Result<SignedRequest> {
    let digest = hmac_sha256(request.secret, request.body)?;
    let header_name = request
        .scheme
        .header()
        .unwrap_or("X-Signature")
        .to_owned();
    let mut headers = BTreeMap::new();
    headers.insert(header_name, hex::encode(digest));
    headers.insert("Content-Type".to_owned(), "application/json".to_owned());
    Ok(SignedRequest { headers })
}

pub(super) fn hubspot_v3(request: &SignRequest<'_>) -> Result<SignedRequest> {
    // HubSpot v3: HMAC-SHA256 over `${method}${uri}${body}${timestamp}` — for
    // outbound delivery we sign POST + path + body + timestamp.
    let url = request.destination_url.ok_or(Error::SigningFailed {
        reason: "hubspot v3 signing requires destination_url",
    })?;
    let path = url
        .split_once("://")
        .and_then(|(_, rest)| rest.find('/').map(|i| &rest[i..]))
        .unwrap_or("/");
    let source = format!(
        "POST{}{}{}",
        path,
        String::from_utf8_lossy(request.body),
        request.timestamp
    );
    let digest = hmac_sha256(request.secret, source.as_bytes())?;
    let mut headers = BTreeMap::new();
    headers.insert("X-HubSpot-Signature-v3".to_owned(), B64.encode(digest));
    headers.insert(
        "X-HubSpot-Request-Timestamp".to_owned(),
        request.timestamp.to_string(),
    );
    headers.insert("Content-Type".to_owned(), "application/json".to_owned());
    Ok(SignedRequest { headers })
}

pub(super) fn twilio(request: &SignRequest<'_>) -> Result<SignedRequest> {
    // Twilio signs URL + sorted POST params. For JSON bodies we sign the URL
    // alone plus the raw body as a single `body` param surrogate used by many
    // JSON-mode receivers; form-encoded callers should prefer URL+params.
    let url = request.destination_url.ok_or(Error::SigningFailed {
        reason: "twilio signing requires destination_url",
    })?;
    let mut data = String::from(url);
    data.push_str(&String::from_utf8_lossy(request.body));
    let digest = hmac_sha1(request.secret, data.as_bytes())?;
    let mut headers = BTreeMap::new();
    headers.insert("X-Twilio-Signature".to_owned(), B64.encode(digest));
    headers.insert(
        "Content-Type".to_owned(),
        "application/x-www-form-urlencoded".to_owned(),
    );
    Ok(SignedRequest { headers })
}

pub(super) fn gitlab_token(request: &SignRequest<'_>) -> Result<SignedRequest> {
    let mut headers = BTreeMap::new();
    headers.insert(
        "X-Gitlab-Token".to_owned(),
        request.secret.expose().to_owned(),
    );
    headers.insert("Content-Type".to_owned(), "application/json".to_owned());
    Ok(SignedRequest { headers })
}
