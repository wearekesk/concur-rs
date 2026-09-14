//! SAP Concur Receipts API v4.
//!
//! Endpoints: `receipts/v4/...`

use crate::client::ConcurClient;
use crate::error::Result;
use reqwest::Method;
use serde::Deserialize;

/// Receipts API.
pub struct ReceiptsApi<'a> {
    client: &'a ConcurClient,
}

impl<'a> ReceiptsApi<'a> {
    pub fn new(client: &'a ConcurClient) -> Self {
        Self { client }
    }

    /// Get service index.
    pub async fn index(&self) -> Result<ReceiptIndex> {
        self.client
            .request(Method::GET, "/receipts/")
            .await?
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// Get supported receipt schemas.
    pub async fn schemas(&self, schema_id: Option<&str>) -> Result<serde_json::Value> {
        let path = match schema_id {
            Some(id) => format!("/receipts/schemas/{}", id),
            None => "/receipts/schemas".to_string(),
        };
        self.client
            .request(Method::GET, &path)
            .await?
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// Get receipt processing status.
    pub async fn status(&self, receipt_id: &str) -> Result<ReceiptStatus> {
        self.client
            .request(Method::GET, &format!("/receipts/v4/status/{}", receipt_id))
            .await?
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// Post a receipt (JSON only).
    pub async fn post_receipt(
        &self,
        user_id: &str,
        receipt: &serde_json::Value,
        schema_url: &str,
    ) -> Result<PostReceiptResponse> {
        let req = self
            .client
            .request(Method::POST, &format!("/receipts/v4/users/{}", user_id))
            .await?;
        let req = req
            .header("Content-Type", "application/json")
            .header("link", format!("<{}>;rel=describedBy", schema_url))
            .json(receipt);
        let response = self.client.execute_with_raw(req).await?;
        let location = response
            .headers()
            .get("location")
            .and_then(|v| v.to_str().ok())
            .map(String::from);
        let link = response
            .headers()
            .get("link")
            .and_then(|v| v.to_str().ok())
            .map(String::from);
        Ok(PostReceiptResponse { location, link })
    }

    /// Get a user's receipts.
    pub async fn get_user_receipts(&self, user_id: &str) -> Result<Vec<serde_json::Value>> {
        self.client
            .request(Method::GET, &format!("/receipts/v4/users/{}", user_id))
            .await?
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// Get a receipt by ID.
    pub async fn get_receipt(&self, receipt_id: &str) -> Result<serde_json::Value> {
        self.client
            .request(Method::GET, &format!("/receipts/v4/{}", receipt_id))
            .await?
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// Get a receipt image.
    pub async fn get_receipt_image(&self, receipt_id: &str) -> Result<Vec<u8>> {
        self.client
            .request(Method::GET, &format!("/receipts/v4/{}/image", receipt_id))
            .await?
            .send()
            .await?
            .bytes()
            .await
            .map(|b| b.to_vec())
            .map_err(Into::into)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReceiptIndex {
    pub links: Vec<ReceiptLink>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReceiptLink {
    pub rel: String,
    pub href: String,
    pub method: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReceiptStatus {
    pub status: String,
    pub logs: Vec<ReceiptLog>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ReceiptLog {
    #[serde(rename = "logLevel")]
    pub log_level: String,
    pub message: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Default)]
pub struct PostReceiptResponse {
    pub location: Option<String>,
    pub link: Option<String>,
}
