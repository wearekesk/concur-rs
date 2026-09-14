//! SAP Concur Purchase Request API v4.
//!
//! Endpoints: `purchaserequest/v4/...`

use crate::client::ConcurClient;
use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// Purchase Request API.
pub struct PurchaseRequestApi<'a> {
    client: &'a ConcurClient,
}

impl<'a> PurchaseRequestApi<'a> {
    pub fn new(client: &'a ConcurClient) -> Self {
        Self { client }
    }

    /// Create a purchase request.
    pub async fn create(&self, request: &CreatePurchaseRequest) -> Result<CreatePurchaseResponse> {
        let req = self
            .client
            .request(Method::POST, "/purchaserequest/v4/purchaserequests")
            .await?;
        let req = ConcurClient::with_json(req, request)?;
        self.client.execute(req).await
    }

    /// Get purchase request details.
    pub async fn get(&self, purchase_request_id: &str) -> Result<PurchaseRequest> {
        self.client
            .request(
                Method::GET,
                &format!(
                    "/purchaserequest/v4/purchaserequests/{}",
                    purchase_request_id
                ),
            )
            .await?
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreatePurchaseRequest {
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    #[serde(rename = "userEmail")]
    pub user_email: Option<String>,
    #[serde(rename = "userLoginId")]
    pub user_login_id: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "policyId")]
    pub policy_id: Option<String>,
    pub currency: Option<String>,
    #[serde(rename = "notesToSupplier")]
    pub notes_to_supplier: Option<String>,
    pub comments: Option<String>,
    #[serde(rename = "custom1", skip_serializing_if = "Option::is_none")]
    pub custom1: Option<String>,
    #[serde(rename = "custom2", skip_serializing_if = "Option::is_none")]
    pub custom2: Option<String>,
    #[serde(rename = "custom3", skip_serializing_if = "Option::is_none")]
    pub custom3: Option<String>,
    #[serde(rename = "custom4", skip_serializing_if = "Option::is_none")]
    pub custom4: Option<String>,
    #[serde(rename = "custom5", skip_serializing_if = "Option::is_none")]
    pub custom5: Option<String>,
    #[serde(rename = "custom6", skip_serializing_if = "Option::is_none")]
    pub custom6: Option<String>,
    #[serde(rename = "custom7", skip_serializing_if = "Option::is_none")]
    pub custom7: Option<String>,
    #[serde(rename = "custom8", skip_serializing_if = "Option::is_none")]
    pub custom8: Option<String>,
    #[serde(rename = "custom9", skip_serializing_if = "Option::is_none")]
    pub custom9: Option<String>,
    #[serde(rename = "custom10", skip_serializing_if = "Option::is_none")]
    pub custom10: Option<String>,
    #[serde(rename = "custom11", skip_serializing_if = "Option::is_none")]
    pub custom11: Option<String>,
    #[serde(rename = "custom12", skip_serializing_if = "Option::is_none")]
    pub custom12: Option<String>,
    #[serde(rename = "custom13", skip_serializing_if = "Option::is_none")]
    pub custom13: Option<String>,
    #[serde(rename = "custom14", skip_serializing_if = "Option::is_none")]
    pub custom14: Option<String>,
    #[serde(rename = "custom15", skip_serializing_if = "Option::is_none")]
    pub custom15: Option<String>,
    #[serde(rename = "custom16", skip_serializing_if = "Option::is_none")]
    pub custom16: Option<String>,
    #[serde(rename = "custom17", skip_serializing_if = "Option::is_none")]
    pub custom17: Option<String>,
    #[serde(rename = "custom18", skip_serializing_if = "Option::is_none")]
    pub custom18: Option<String>,
    #[serde(rename = "custom19", skip_serializing_if = "Option::is_none")]
    pub custom19: Option<String>,
    #[serde(rename = "custom20", skip_serializing_if = "Option::is_none")]
    pub custom20: Option<String>,
    #[serde(rename = "lineItems")]
    pub line_items: Vec<PurchaseRequestLineItem>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PurchaseRequestLineItem {
    #[serde(rename = "expenseTypeId")]
    pub expense_type_id: Option<String>,
    pub description: Option<String>,
    pub quantity: Option<f64>,
    #[serde(rename = "unitPrice")]
    pub unit_price: Option<f64>,
    pub currency: Option<String>,
    #[serde(rename = "vendorCode")]
    pub vendor_code: Option<String>,
    #[serde(rename = "vendorAddressCode")]
    pub vendor_address_code: Option<String>,
    #[serde(rename = "shipToAddressCode")]
    pub ship_to_address_code: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreatePurchaseResponse {
    pub id: String,
    pub uri: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PurchaseRequest {
    pub id: String,
    #[serde(rename = "purchaseRequestNumber")]
    pub purchase_request_number: Option<String>,
    #[serde(rename = "purchaseOrderNumber")]
    pub purchase_order_number: Option<String>,
    #[serde(rename = "workflowStatus")]
    pub workflow_status: Option<String>,
    pub exceptions: Option<Vec<serde_json::Value>>,
}
