# concur-api

[![CI](https://github.com/wearekesk/concur-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/wearekesk/concur-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/concur-api.svg)](https://crates.io/crates/concur-api)
[![Docs.rs](https://docs.rs/concur-api/badge.svg)](https://docs.rs/concur-api)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A typed, async Rust client for the [SAP Concur](https://developer.concur.com/) REST APIs. This crate targets the latest generally available **v4 / v4.1** endpoints and avoids deprecated v3 APIs where a v4 successor exists.

## Supported APIs

| Area | API | Version | Base path |
|------|-----|---------|-----------|
| Authentication | OAuth2 / App Authorization | v0 | `oauth2/v0/token` |
| Identity | Identity API | **v4.1** | `profile/identity/v4.1` |
| Expense | Expense Reports API | v4 | `expensereports/v4/.../reports` |
| Expense | Expenses (entries) API | v4 | `expensereports/v4/.../expenses` |
| Expense | Expense Workflow API | v4 | `expensereports/v4/.../approve` etc. |
| Configuration | Lists API | v4 | `list/v4/lists` |
| Configuration | List Items API | v4 | `list/v4/items` |
| Receipts | Receipts API | v4 | `receipts/v4/...` |
| Request | Travel Request API | v4 | `travelrequest/v4/requests` |
| Request | Request Workflow API | v4 | `travelrequest/v4/requests/{id}/{action}` |
| Request | Expected Expenses API | v4 | `travelrequest/v4/requests/{id}/expenses` |
| Request | Request Allocations API | v4 | `travelrequest/v4/requests/{id}/allocations` |
| Cards | Cards API | v4 | `cards/v4/companies/{id}/accounts/bulk` |
| Cards | Card Transactions API | v4 | `cards/v4/companies/{id}/transactions/bulk` |
| Purchasing | Purchase Request API | v4 | `purchaserequest/v4/...` |
| Invoice | Invoice Pay API | v4 | `invoice/provider-payment/v4/payments` |
| Events | Event Subscription Service (ESS) | v4 | `events/v4/...` |

### APIs intentionally excluded

The following APIs were requested but **do not have a v4 endpoint** (or the v4 endpoint is not publicly documented) as of the latest SAP Concur Developer Center review:

* Expense Allocations API — only v3 exists (`/api/v3.0/expense/allocations`).
* Expense Attendees API — only v1/v2/v3 exist.
* Expense Itemizations API — only v3 exists; itemizations are exposed as links inside Expenses v4.
* Company / Organization API — v1 is deprecated (May 2025); company context is available via Identity v4.1 user profiles.
* General Invoice API — only v3 exists; v4 is limited to **Invoice Pay** and **Invoice Payment Confirmation**.
* Purchase Order APIs — only v3 exists.
* Cost Centers / Custom Fields APIs — no v4 endpoints found.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
concur-api = "0.1"
```

## Quick start

```rust
use concur_api::{ClientConfig, ConcurClient, Result};
use concur_api::auth::{OAuth2TokenManager, Grant};
use concur_api::api::expense::ReportsApi;

#[tokio::main]
async fn main() -> Result<()> {
    let config = ClientConfig {
        base_url: "https://us.api.concursolutions.com".to_string(),
        ..Default::default()
    };

    let manager = OAuth2TokenManager::new(
        "https://us.api.concursolutions.com/oauth2/v0/token",
        "your-client-id",
        "your-client-secret",
        Grant::ClientCredentials {
            scope: Some("expense.report.read".to_string()),
        },
    );

    let client = ConcurClient::new(config)?.with_token_manager(manager);
    let reports = ReportsApi::new(&client);

    let report = reports.get(
        "32c2fcc3-b2e8-4907-9672-5b3f49b1c643",
        "TRAVELER",
        "764428DD6A664AF0BFCB",
    ).await?;

    println!("{:?}", report);
    Ok(())
}
```

## Authentication

The crate supports multiple OAuth2 flows:

* `Grant::ClientCredentials` — company / application tokens.
* `Grant::Password` — user tokens (including App Center `authtoken` credential type).
* `Grant::RefreshToken` — refresh an existing token.
* `StaticToken` — for testing or when you already have a token.

Token refresh is handled automatically by `OAuth2TokenManager` before the token expires.

## Features

* Async/await API built on `reqwest` and `tokio`.
* Automatic token refresh and geolocation-aware base URLs.
* Exponential backoff retry for transient HTTP errors (429, 5xx).
* `concur-correlationid` header generation for request tracing.
* Strongly typed request/response models for all supported APIs.
* ESS webhook subscription management.

## Geolocation

SAP Concur returns a `geolocation` field in the OAuth2 token response. You should configure `ClientConfig::base_url` with that value for subsequent API calls. The client does not automatically switch data centers; store the geolocation from the token response and create a new `ConcurClient` for that base URL.

## Rate limits and retries

The client retries idempotent GET/PUT/DELETE requests automatically on 429 / 5xx responses. Non-idempotent POST/PATCH requests are also retried, but callers should supply an idempotency key where the API supports it. Configure retry behavior via `ClientConfig::max_retries` and `ClientConfig::retry_backoff`.

## Documentation

* [SAP Concur Developer Center](https://developer.concur.com/)
* [API Reference](https://developer.concur.com/api-reference/)
* [crate docs](https://docs.rs/concur-api)

## License

This project is licensed under the [MIT License](LICENSE-MIT).

## Authors

* Balamurali Pandranki <balamurali@live.com>
* Kesk Open Source <opensource@kesk.app>
