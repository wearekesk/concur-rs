//! SAP Concur Identity API v4.1.
//!
//! Endpoints: `profile/identity/v4.1/Users`

use crate::client::ConcurClient;
use crate::error::Result;
use crate::models::{Address, Email, Meta, UserName};
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// Identity API resource.
pub struct IdentityApi<'a> {
    client: &'a ConcurClient,
}

impl<'a> IdentityApi<'a> {
    pub fn new(client: &'a ConcurClient) -> Self {
        Self { client }
    }

    /// List users for the company.
    pub async fn list_users(&self, count: Option<u32>, cursor: Option<&str>) -> Result<UserList> {
        let mut path = "/profile/identity/v4.1/Users".to_string();
        let mut sep = '?';
        if let Some(c) = count {
            path.push_str(&format!("{}count={}", sep, c));
            sep = '&';
        }
        if let Some(c) = cursor {
            path.push_str(&format!("{}cursor={}", sep, urlencoding::encode(c)));
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

    /// Search users via POST `/.search`.
    pub async fn search_users(
        &self,
        filter: &str,
        attributes: Option<Vec<String>>,
    ) -> Result<UserList> {
        let body = SearchRequest {
            schemas: vec!["urn:ietf:params:scim:api:messages:concur:2.0:SearchRequest".to_string()],
            filter: filter.to_string(),
            attributes,
        };
        let req = self
            .client
            .request(Method::POST, "/profile/identity/v4.1/Users/.search")
            .await?;
        let req = ConcurClient::with_json(req, &body)?;
        self.client.execute(req).await
    }

    /// Get a user by UUID.
    pub async fn get_user(&self, id: &str) -> Result<User> {
        self.client
            .request(Method::GET, &format!("/profile/identity/v4.1/Users/{}", id))
            .await?
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// Create a new user.
    pub async fn create_user(&self, user: &User) -> Result<User> {
        let req = self
            .client
            .request(Method::POST, "/profile/identity/v4.1/Users")
            .await?;
        let req = ConcurClient::with_json(req, user)?;
        self.client.execute(req).await
    }

    /// Update a user with PATCH (RFC 7644).
    pub async fn patch_user(&self, id: &str, ops: &[PatchOp]) -> Result<User> {
        let body = PatchRequest {
            schemas: vec!["urn:ietf:params:scim:api:messages:2.0:PatchOp".to_string()],
            operations: ops.to_vec(),
        };
        let req = self
            .client
            .request(
                Method::PATCH,
                &format!("/profile/identity/v4.1/Users/{}", id),
            )
            .await?;
        let req = ConcurClient::with_json(req, &body)?;
        self.client.execute(req).await
    }

    /// Delete a user.
    pub async fn delete_user(&self, id: &str) -> Result<()> {
        self.client
            .execute_unit(
                self.client
                    .request(
                        Method::DELETE,
                        &format!("/profile/identity/v4.1/Users/{}", id),
                    )
                    .await?,
            )
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    pub schemas: Vec<String>,
    pub filter: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRequest {
    pub schemas: Vec<String>,
    pub operations: Vec<PatchOp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchOp {
    pub op: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserList {
    pub schemas: Vec<String>,
    #[serde(rename = "totalResults")]
    pub total_results: u64,
    #[serde(rename = "itemsPerPage")]
    pub items_per_page: u32,
    pub resources: Vec<User>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct User {
    pub schemas: Vec<String>,
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<UserName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<Email>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<Address>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(
        rename = "urn:ietf:params:scim:schemas:extension:enterprise:2.0:User",
        skip_serializing_if = "Option::is_none"
    )]
    pub enterprise_extension: Option<EnterpriseExtension>,
    #[serde(
        rename = "urn:ietf:params:scim:schemas:extension:sap:2.0:User",
        skip_serializing_if = "Option::is_none"
    )]
    pub sap_extension: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnterpriseExtension {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_date: Option<String>,
}
