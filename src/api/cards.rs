//! SAP Concur Cards API v4 and Card Transactions API v4.
//!
//! Endpoints: `cards/v4/...`

use crate::client::ConcurClient;
use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// Cards API.
pub struct CardsApi<'a> {
    client: &'a ConcurClient,
}

impl<'a> CardsApi<'a> {
    pub fn new(client: &'a ConcurClient) -> Self {
        Self { client }
    }

    /// Create card accounts in bulk.
    pub async fn create_accounts_bulk(
        &self,
        company_id: &str,
        request: &CreateAccountsRequest,
    ) -> Result<BulkResponse> {
        let req = self
            .client
            .request(
                Method::POST,
                &format!("/cards/v4/companies/{}/accounts/bulk", company_id),
            )
            .await?;
        let req = ConcurClient::with_json(req, request)?;
        self.client.execute(req).await
    }

    /// Create card transactions in bulk.
    pub async fn create_transactions_bulk(
        &self,
        company_id: &str,
        request: &CreateTransactionsRequest,
    ) -> Result<BulkResponse> {
        let req = self
            .client
            .request(
                Method::POST,
                &format!("/cards/v4/companies/{}/transactions/bulk", company_id),
            )
            .await?;
        let req = ConcurClient::with_json(req, request)?;
        self.client.execute(req).await
    }

    /// Get bulk request details.
    pub async fn get_bulk_request(
        &self,
        company_id: &str,
        request_id: &str,
    ) -> Result<BulkRequestStatus> {
        self.client
            .request(
                Method::GET,
                &format!(
                    "/cards/v4/companies/{}/bulkrequests/{}",
                    company_id, request_id
                ),
            )
            .await?
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// Retrieve card transactions for a user.
    pub async fn get_user_transactions(
        &self,
        user_id: &str,
        query: &TransactionQuery,
    ) -> Result<TransactionListResponse> {
        let mut path = format!("/cards/v4/users/{}/transactions", user_id);
        let mut params: Vec<(String, String)> = Vec::new();
        if let Some(v) = &query.status {
            params.push(("status".to_string(), v.clone()));
        }
        if let Some(v) = &query.transaction_date_from {
            params.push(("transactionDateFrom".to_string(), v.clone()));
        }
        if let Some(v) = &query.transaction_date_to {
            params.push(("transactionDateTo".to_string(), v.clone()));
        }
        if let Some(v) = query.page_size {
            params.push(("pageSize".to_string(), v.to_string()));
        }
        if let Some(v) = &query.page_token {
            params.push(("pageToken".to_string(), v.clone()));
        }
        if let Some(v) = &query.sort {
            params.push(("sort".to_string(), v.clone()));
        }
        if let Some(v) = &query.order {
            params.push(("order".to_string(), v.clone()));
        }
        if let Some(v) = &query.include_addendum_for {
            params.push(("includeAddendumFor".to_string(), v.clone()));
        }
        if let Some(v) = &query.include_addendum_details_for {
            params.push(("includeAddendumDetailsFor".to_string(), v.clone()));
        }
        if !params.is_empty() {
            path.push('?');
            path.push_str(
                &params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
                    .collect::<Vec<_>>()
                    .join("&"),
            );
        }
        self.client
            .request(Method::GET, &path)
            .await?
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }
}

#[derive(Debug, Clone, Default)]
pub struct TransactionQuery {
    pub status: Option<String>,
    pub transaction_date_from: Option<String>,
    pub transaction_date_to: Option<String>,
    pub page_size: Option<u32>,
    pub page_token: Option<String>,
    pub sort: Option<String>,
    pub order: Option<String>,
    pub include_addendum_for: Option<String>,
    pub include_addendum_details_for: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateAccountsRequest {
    pub accounts: Vec<CardAccount>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardAccount {
    #[serde(rename = "accountType")]
    pub account_type: String,
    #[serde(rename = "billedCurrency")]
    pub billed_currency: String,
    #[serde(rename = "cardBrand")]
    pub card_brand: String,
    #[serde(rename = "cardProductDescription")]
    pub card_product_description: Option<String>,
    #[serde(rename = "cardProductType")]
    pub card_product_type: String,
    pub cardholder: Cardholder,
    #[serde(rename = "externalId")]
    pub external_id: String,
    #[serde(rename = "lastSegment")]
    pub last_segment: Option<String>,
    #[serde(rename = "liabilityType")]
    pub liability_type: String,
    #[serde(rename = "nameOnCard")]
    pub name_on_card: String,
    pub provider: CardProvider,
    pub status: String,
    #[serde(rename = "billingAccount", skip_serializing_if = "Option::is_none")]
    pub billing_account: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cardholder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardProvider {
    #[serde(rename = "countryCode")]
    pub country_code: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateTransactionsRequest {
    pub transactions: Vec<CardTransaction>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardTransaction {
    #[serde(rename = "externalId")]
    pub external_id: String,
    #[serde(rename = "transactionDate")]
    pub transaction_date: String,
    #[serde(rename = "transactionDateTime")]
    pub transaction_date_time: Option<String>,
    #[serde(rename = "postedDate")]
    pub posted_date: String,
    #[serde(rename = "transactionAmount")]
    pub transaction_amount: Money,
    #[serde(rename = "postedAmount")]
    pub posted_amount: Money,
    #[serde(rename = "billedAmount")]
    pub billed_amount: Option<Money>,
    #[serde(rename = "referenceNumber")]
    pub reference_number: Option<String>,
    #[serde(
        rename = "authorizationExternalId",
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_external_id: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    pub description: Option<String>,
    pub account: TransactionAccountRef,
    pub merchant: Option<serde_json::Value>,
    pub lodging: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransactionAccountRef {
    #[serde(rename = "externalId")]
    pub external_id: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Money {
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    pub value: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BulkResponse {
    #[serde(rename = "requestId")]
    pub request_id: String,
    pub links: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BulkRequestStatus {
    #[serde(rename = "requestId")]
    pub request_id: String,
    pub status: String,
    #[serde(rename = "processedCount")]
    pub processed_count: Option<i32>,
    #[serde(rename = "totalCount")]
    pub total_count: Option<i32>,
    pub errors: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TransactionListResponse {
    pub transactions: Vec<serde_json::Value>,
    pub paging: Option<serde_json::Value>,
}
