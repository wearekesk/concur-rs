//! SAP Concur Invoice Pay API v4.
//!
//! Endpoints: `invoice/provider-payment/v4/...`

use crate::client::ConcurClient;
use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// Invoice Pay API.
pub struct InvoicePayApi<'a> {
    client: &'a ConcurClient,
}

impl<'a> InvoicePayApi<'a> {
    pub fn new(client: &'a ConcurClient) -> Self {
        Self { client }
    }

    /// List pending payments.
    pub async fn list_payments(&self, query: &PaymentQuery) -> Result<PaymentsResponse> {
        let mut path = "/invoice/provider-payment/v4/payments".to_string();
        let mut params: Vec<(String, String)> = Vec::new();
        if let Some(v) = &query.invoice_id {
            params.push(("invoiceId".to_string(), v.clone()));
        }
        if let Some(v) = &query.invoice_date {
            params.push(("invoiceDate".to_string(), v.clone()));
        }
        if let Some(v) = &query.create_date {
            params.push(("createDate".to_string(), v.clone()));
        }
        if let Some(v) = &query.vendor_name {
            params.push(("vendorName".to_string(), v.clone()));
        }
        if let Some(v) = &query.vendor_code {
            params.push(("vendorCode".to_string(), v.clone()));
        }
        if let Some(v) = &query.vendor_addr_code {
            params.push(("vendorAddrCode".to_string(), v.clone()));
        }
        if let Some(v) = &query.payment_status {
            params.push(("PaymentStatus".to_string(), v.clone()));
        }
        if let Some(v) = &query.payment_group {
            params.push(("PaymentGroup".to_string(), v.clone()));
        }
        if let Some(v) = &query.invoice_number {
            params.push(("invoiceNumber".to_string(), v.clone()));
        }
        if let Some(v) = &query.payment_id {
            params.push(("paymentId".to_string(), v.clone()));
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

    /// Update a single payment status.
    pub async fn update_payment(
        &self,
        payment_id: &str,
        update: &PaymentUpdate,
    ) -> Result<PaymentUpdateResult> {
        let req = self
            .client
            .request(
                Method::POST,
                &format!("/invoice/provider-payment/v4/payments/{}", payment_id),
            )
            .await?;
        let req = ConcurClient::with_json(req, update)?;
        self.client.execute(req).await
    }

    /// Update multiple payment statuses in bulk.
    pub async fn bulk_update_payments(
        &self,
        updates: &[PaymentUpdate],
    ) -> Result<BulkUpdateResult> {
        let req = self
            .client
            .request(
                Method::POST,
                "/invoice/provider-payment/v4/payments/bulkUpdate",
            )
            .await?;
        let req = ConcurClient::with_json(req, updates)?;
        self.client.execute(req).await
    }
}

#[derive(Debug, Clone, Default)]
pub struct PaymentQuery {
    pub invoice_id: Option<String>,
    pub invoice_date: Option<String>,
    pub create_date: Option<String>,
    pub vendor_name: Option<String>,
    pub vendor_code: Option<String>,
    pub vendor_addr_code: Option<String>,
    pub payment_status: Option<String>,
    pub payment_group: Option<String>,
    pub invoice_number: Option<String>,
    pub payment_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PaymentsResponse {
    pub payments: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentUpdate {
    #[serde(rename = "paymentId")]
    pub payment_id: Option<String>,
    #[serde(rename = "providerReference")]
    pub provider_reference: Option<String>,
    pub status: String,
    #[serde(rename = "statusMessage")]
    pub status_message: Option<String>,
    #[serde(rename = "statusDate")]
    pub status_date: Option<String>,
    #[serde(rename = "paymentInitiationDate")]
    pub payment_initiation_date: Option<String>,
    #[serde(rename = "paymentSettlementDate")]
    pub payment_settlement_date: Option<String>,
    #[serde(rename = "thirdPartyPaymentIdentifier")]
    pub third_party_payment_identifier: Option<String>,
    #[serde(rename = "paymentMethod")]
    pub payment_method: Option<String>,
    #[serde(rename = "paidAmount")]
    pub paid_amount: Option<Money>,
    #[serde(rename = "paymentAdjustmentNotes")]
    pub payment_adjustment_notes: Option<String>,
    #[serde(rename = "fundingSourceRef")]
    pub funding_source_ref: Option<String>,
    #[serde(rename = "cashAccountCode")]
    pub cash_account_code: Option<String>,
    #[serde(rename = "liabilityAccountCode")]
    pub liability_account_code: Option<String>,
    #[serde(rename = "fundingRequestRef")]
    pub funding_request_ref: Option<String>,
    #[serde(rename = "checkNumber")]
    pub check_number: Option<String>,
    #[serde(rename = "fundingSettlementDate")]
    pub funding_settlement_date: Option<String>,
    #[serde(rename = "fundingCurrency")]
    pub funding_currency: Option<String>,
    #[serde(rename = "paymentCurrency")]
    pub payment_currency: Option<String>,
    #[serde(rename = "foreignExchangeRate")]
    pub foreign_exchange_rate: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Money {
    pub amount: String,
    pub currency: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PaymentUpdateResult {
    #[serde(rename = "paymentId")]
    pub payment_id: String,
    pub status: String,
    #[serde(rename = "statusMessage")]
    pub status_message: Option<String>,
    #[serde(rename = "statusDate")]
    pub status_date: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BulkUpdateResult {
    #[serde(rename = "successfulPayments")]
    pub successful_payments: Option<Vec<PaymentUpdateResult>>,
    #[serde(rename = "failedPayments")]
    pub failed_payments: Option<Vec<serde_json::Value>>,
}
