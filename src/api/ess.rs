//! SAP Concur Event Subscription Service (ESS) v4.
//!
//! Endpoints: `events/v4/...`

use crate::client::ConcurClient;
use crate::error::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// Event Subscription Service API.
pub struct EssApi<'a> {
    client: &'a ConcurClient,
}

impl<'a> EssApi<'a> {
    pub fn new(client: &'a ConcurClient) -> Self {
        Self { client }
    }

    /// Browse available topics.
    pub async fn topics(&self) -> Result<Vec<String>> {
        self.client
            .request(Method::GET, "/events/v4/topics")
            .await?
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// Create or update a webhook subscription.
    pub async fn create_subscription(
        &self,
        subscription: &Subscription,
    ) -> Result<SubscriptionResult> {
        let req = self
            .client
            .request(Method::PUT, "/events/v4/subscriptions/webhook")
            .await?;
        let req = ConcurClient::with_json(req, subscription)?;
        self.client.execute(req).await
    }

    /// Get a subscription by ID.
    pub async fn get_subscription(&self, subscription_id: &str) -> Result<Vec<Subscription>> {
        self.client
            .request(
                Method::GET,
                &format!("/events/v4/subscriptions/{}", subscription_id),
            )
            .await?
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// List all subscriptions.
    pub async fn list_subscriptions(&self) -> Result<Vec<Subscription>> {
        self.client
            .request(Method::GET, "/events/v4/subscriptions")
            .await?
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// Delete a subscription.
    pub async fn delete_subscription(&self, subscription_id: &str) -> Result<SubscriptionResult> {
        self.client
            .execute_unit(
                self.client
                    .request(
                        Method::DELETE,
                        &format!("/events/v4/subscriptions/{}", subscription_id),
                    )
                    .await?,
            )
            .await?;
        Ok(SubscriptionResult {
            message: format!("Subscription '{}' marked for deletion", subscription_id),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub filter: String,
    pub topic: String,
    #[serde(rename = "webHookConfig")]
    pub webhook_config: WebhookConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "applicationId")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "companyIds")]
    pub company_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub endpoint: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubscriptionResult {
    pub message: String,
}

/// Common ESS event envelope.
#[derive(Debug, Clone, Deserialize)]
pub struct EssEvent<T> {
    #[serde(rename = "eventType")]
    pub event_type: String,
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
    #[serde(rename = "correlationId")]
    pub correlation_id: Option<String>,
    #[serde(rename = "facts")]
    pub facts: T,
}
