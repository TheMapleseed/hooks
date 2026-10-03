# hooks

Swiss-army-knife Rust crate for **HTTP webhooks** (Stripe, GitHub, Slack, Shopify, and other common providers).

Compose the right webhook type in code (no embedded JSON/YAML templates), **register** it with the provider API when supported, then **sign and deliver** payloads to the correct destination.

```text
describe  →  register  →  deliver
 (typed)     (provider API)   (HMAC / token + HTTP POST)
```

| Layer | What it does |
| --- | --- |
| **Describe** | Typed builders for HTTP webhook endpoints (Stripe, GitHub, Slack, …) |
| **Register** | Create remote endpoints (`StripeRegistrar`, `GitHubRegistrar`, …) |
| **Deliver** | Provider-correct signing + blocking HTTP POST to a receiver URL |
| **Conform** | Build-gated suite that checks wire shapes against each API’s standards |

Edition **2024**. Prefer **KDL** for human-edited source docs; JSON emit is also available.

---

## Install

```toml
[dependencies]
hooks = { git = "https://github.com/TheMapleseed/Hooks" }
```

```sh
cargo add hooks --git https://github.com/TheMapleseed/Hooks
```

---

## Quick start

### 1. Describe a webhook (Stripe)

```rust
use hooks::stripe::{self, Event};

let endpoint = stripe::endpoint()
    .url("https://api.example.com/webhooks/stripe")
    .events([
        Event::PaymentIntentSucceeded,
        Event::CheckoutSessionCompleted,
        Event::InvoicePaid,
    ])
    .api_version("2024-11-20.acacia")
    .secret_env("STRIPE_WEBHOOK_SECRET") // env *name* only — never the secret itself
    .description("Production payments")
    .build()?;

println!("{}", endpoint.to_kdl()?);  // preferred source form
println!("{}", endpoint.to_json()?);
```

Same pattern for other providers: `hooks::github`, `slack`, `shopify`, `clerk`, …

### 2. Register with the provider API

**Stripe** — `POST /v1/webhook_endpoints`:

```rust
use hooks::register::{Registrar, StripeRegistrar};
use hooks::stripe::{self, Event};
use hooks::Secret;

let webhook = stripe::endpoint()
    .url("https://api.example.com/webhooks/stripe")
    .events([Event::CheckoutSessionCompleted])
    .api_version("2024-11-20.acacia")
    .build()?;

let registered = StripeRegistrar::new(Secret::from_env("STRIPE_API_KEY")?)
    .register(&webhook)?;

// Store this immediately in a secret manager — it is not written to disk.
let _ = registered.signing_secret;
```

**GitHub** — `POST /repos/{owner}/{repo}/hooks`:

```rust
use hooks::register::{GitHubRegistrar, Registrar};
use hooks::github::{self, Event};
use hooks::Secret;

let webhook = github::endpoint()
    .url("https://api.example.com/webhooks/github")
    .events([Event::Push, Event::PullRequest])
    .secret_env("GITHUB_WEBHOOK_SECRET")
    .build()?;

let registered = GitHubRegistrar::new(
    Secret::from_env("GITHUB_TOKEN")?,
    "TheMapleseed",
    "Hooks",
)
.register(&webhook)?;
```

**Unified client:**

```rust
use hooks::Client;

let client = Client::new();
client.register_stripe(Secret::from_env("STRIPE_API_KEY")?, &webhook)?;
client.register_github(Secret::from_env("GITHUB_TOKEN")?, "owner", "repo", &webhook)?;
```

### 3. Sign and deliver a payload

```rust
use hooks::deliver::{envelope_stripe, Delivery, DeliveryClient, OutboundEvent};
use hooks::{Provider, Secret};
use serde_json::json;

let secret = Secret::from_env("STRIPE_WEBHOOK_SECRET")?;

let event = OutboundEvent::new(
    "payment_intent.succeeded",
    json!({"id": "pi_123", "object": "payment_intent", "amount": 1000}),
);
let body = serde_json::to_vec(&envelope_stripe(&event))?;

let delivery = Delivery::builder(Provider::Stripe)
    .url("https://api.example.com/webhooks/stripe")
    .event(event.event)
    .body(body)
    .build()?;

let response = DeliveryClient::new().send(&delivery, &secret)?;
assert!((200..300).contains(&response.status));
```

Or target a previously built `Webhook`:

```rust
use hooks::{Client, Secret};

Client::new().send_to_webhook(&endpoint, "payment_intent.succeeded", body, &secret)?;
```

Live demo against a local receiver:

```sh
HOOKS_DEMO_URL=http://127.0.0.1:9999/hook cargo run --example swiss_army
```

---

## Mental model

### Describe (typed surface)

`WebhookBuilder` / `stripe::endpoint()` → `Webhook` describing URL, events, signature metadata, optional `secret_env`.

Secrets are **never** stored in emitted config. Use `secret_env("VAR_NAME")` plus `Secret::from_env` at runtime.

### Register

| Target | API |
| --- | --- |
| Stripe | `StripeRegistrar` |
| GitHub | `GitHubRegistrar` |

Other providers can be described and delivered today; more registrars are expanding (see conformance skips).

### Deliver

1. Build body (raw JSON or a helper envelope like `envelope_stripe`)  
2. `sign()` applies the provider’s scheme  
3. `DeliveryClient::send` POSTs body + headers to the destination  

Supported signing today: Stripe, GitHub/Bitbucket, Slack, Shopify, Standard Webhooks/Svix (Clerk), Linear, Vercel, HubSpot v3, Twilio, GitLab token. Discord Ed25519 and PayPal certificate verification are not outbound-signed yet.

---

## Providers

| Provider | Module | Signature scheme | Register helper | Envelope helper |
| --- | --- | --- | --- | --- |
| Stripe | `hooks::stripe` | `Stripe-Signature` | `StripeRegistrar` | `envelope_stripe` |
| GitHub | `hooks::github` | `X-Hub-Signature-256` | `GitHubRegistrar` | raw body + event header |
| GitLab | `hooks::gitlab` | `X-Gitlab-Token` | — | — |
| Bitbucket | `hooks::bitbucket` | `X-Hub-Signature` | — | — |
| Slack | `hooks::slack` | `X-Slack-Signature` | — | `envelope_slack_event` |
| Discord | `hooks::discord` | Ed25519 (verify-oriented) | — | — |
| Shopify | `hooks::shopify` | `X-Shopify-Hmac-Sha256` | — | — |
| Twilio | `hooks::twilio` | `X-Twilio-Signature` | — | — |
| SendGrid | `hooks::sendgrid` | IP allowlist | — | — |
| Linear | `hooks::linear` | `Linear-Signature` | — | — |
| Clerk | `hooks::clerk` | Svix / Standard Webhooks | — | — |
| Vercel | `hooks::vercel` | `x-vercel-signature` | — | — |
| HubSpot | `hooks::hubspot` | `X-HubSpot-Signature-v3` | — | — |
| PayPal | `hooks::paypal` | Transmission / cert | — | — |
| PagerDuty | `hooks::pagerduty` | provider signature | — | — |
| Standard Webhooks | `hooks::standard_webhooks` | `webhook-signature` | — | — |

```rust
use hooks::Provider;
for p in Provider::all() {
    println!("{} → {}", p.as_str(), p.signature_scheme().as_str());
}
```

---

## Secrets

```rust
use hooks::Secret;

let from_env = Secret::from_env("STRIPE_WEBHOOK_SECRET")?;
let inline = Secret::new("whsec_…"); // prefer env / secret manager in production
```

- `Debug` redacts values (`Secret([REDACTED])`).  
- Emitted KDL/JSON only stores **environment variable names**.  
- GitHub registration reads the webhook signing secret from `secret_env` at register time.

---

## Conformance suite (build gate)

Checks webhook shapes against each provider’s API standards so the build fails on wire drift.

### What is checked

| Check family | Examples |
| --- | --- |
| **Catalog** | Typed event wire-names match provider naming rules |
| **Signature metadata** | Scheme + primary header match the provider |
| **Signature value shape** | Produced headers match regex (Stripe `t=…,v1=…`, GitHub `sha256=…`) |
| **Payload / envelope** | Required JSON keys (Stripe Event object, Slack `event_callback`) |
| **Registration shape** | Stripe form fields, GitHub `/hooks` JSON body |

### How to run

```sh
cargo test
cargo run --example conformance
HOOKS_CONFORMANCE_VERBOSE=1 cargo run --example conformance
```

```rust
use hooks::{run_suite, assert_suite_passes};

let report = run_suite();
eprintln!("{}", report.summary());
assert_suite_passes(&report);
```

### Why some checks are skipped

A **Skip** is not a hidden failure. It means there is no assertable fixture for that layer yet (or signing is truly N/A).

| Skip reason | Meaning |
| --- | --- |
| **No payload envelope builder** | Envelopes validated only where we synthesize a standard body (Stripe / Slack today) |
| **Registration shape not asserted** | Offline registrar fixtures exist for Stripe + GitHub so far |
| **Signing not implemented / N/A** | Discord Ed25519, PayPal certs, PagerDuty, SendGrid IP allowlist |

**Pass** = shape matches the coded standard.  
**Fail** = build must break.  
**Skip** = backlog / true N/A.

Standards live in `src/conformance/`. The example prints a skip breakdown by default.

---

## Examples

| Example | Purpose |
| --- | --- |
| `cargo run --example catalog -- ./out` | Emit Stripe/GitHub/Slack/Shopify KDL samples |
| `cargo run --example swiss_army` | Sign headers; optional live POST via `HOOKS_DEMO_URL` |
| `cargo run --example conformance` | Full API-shape gate |

---

## Testing locally

```sh
cargo test
cargo clippy --all-targets -- -D warnings
cargo run --example conformance
```

---

## Crate layout

```text
src/
  providers/     Typed events per HTTP provider
  common/        Shared Webhook builder + signature metadata
  sign/          HMAC / token signing
  deliver/       Outbound POST + envelope helpers
  register/      Stripe + GitHub registration
  conformance/   API-standard shape harness
  client.rs      High-level describe → register → deliver façade
  secret.rs      Redacted secret handling
```

---

## Security notes

- Prefer `Secret::from_env` over inlining secrets.  
- Never log exposed secrets or Stripe signing secrets from registration responses.  
- Delivery uses `reqwest` + **rustls**. Local `http://` is allowed for development; production should be `https://`.  
- This crate **sends and registers** webhooks. Inbound signature verification in your receiver is separate.

---

## License

MIT OR Apache-2.0
