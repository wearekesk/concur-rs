//! # SAP Concur API Client for Rust
//!
//! A typed, async client for SAP Concur REST APIs. This crate targets the
//! latest generally available v4 / v4.1 endpoints and avoids deprecated v3
//! APIs where a v4 successor exists.
//!
//! ## Supported APIs
//!
//! * Authentication / OAuth2 (`oauth2/v0/token`)
//! * Identity v4.1 (`profile/identity/v4.1`)
//! * Expense Reports v4
//! * Expenses (entries) v4
//! * Expense Workflow v4
//! * Lists v4 and List Items v4
//! * Receipts v4
//! * Travel Request v4, Request Workflow v4, Expected Expenses v4, Allocations v4
//! * Cards v4 and Card Transactions v4
//! * Purchase Request v4
//! * Invoice Pay v4
//! * Event Subscription Service (ESS) v4
//!
//! ## Example
//!
//! ```no_run
//! use concur_api::{ClientConfig, ConcurClient};
//! use concur_api::auth::{OAuth2TokenManager, Grant};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let config = ClientConfig::default();
//! let client = ConcurClient::new(config)?;
//! let manager = OAuth2TokenManager::new(
//!     "https://us.api.concursolutions.com/oauth2/v0/token",
//!     "client-id",
//!     "client-secret",
//!     Grant::ClientCredentials { scope: Some("expense.report.read".to_string()) },
//! );
//! let client = client.with_token_manager(manager);
//! let reports = concur_api::api::expense::ReportsApi::new(&client);
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod auth;
pub mod client;
pub mod error;
pub mod models;

pub use client::{ClientConfig, ConcurClient};
pub use error::{ConcurError, Result};

/// Convenience re-exports for the most common types.
pub mod prelude {
    pub use crate::api::expense::{
        CreateExpense, CreateReport, ExpensesApi, ReportsApi, WorkflowApi,
    };
    pub use crate::api::identity::IdentityApi;
    pub use crate::api::lists::{ListItemsApi, ListsApi};
    pub use crate::api::receipts::ReceiptsApi;
    pub use crate::api::request::RequestApi;
    pub use crate::auth::{Grant, OAuth2TokenManager, StaticToken, TokenManager};
    pub use crate::client::{ClientConfig, ConcurClient};
    pub use crate::error::{ConcurError, Result};
}
