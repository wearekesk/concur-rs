//! SAP Concur Lists API v4 and List Items API v4.

use crate::client::ConcurClient;
use crate::error::Result;
use crate::models::PagedResponse;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// Lists API.
pub struct ListsApi<'a> {
    client: &'a ConcurClient,
}

impl<'a> ListsApi<'a> {
    pub fn new(client: &'a ConcurClient) -> Self {
        Self { client }
    }

    /// Get all lists with optional filters.
    pub async fn list(
        &self,
        page: Option<u32>,
        sort_by: Option<&str>,
        sort_direction: Option<&str>,
        filters: Option<Vec<(&str, &str)>>,
    ) -> Result<PagedResponse<List>> {
        let mut path = "/list/v4/lists".to_string();
        let mut params: Vec<(String, String)> = Vec::new();
        if let Some(p) = page {
            params.push(("page".to_string(), p.to_string()));
        }
        if let Some(s) = sort_by {
            params.push(("sortBy".to_string(), s.to_string()));
        }
        if let Some(s) = sort_direction {
            params.push(("sortDirection".to_string(), s.to_string()));
        }
        if let Some(f) = filters {
            for (k, v) in f {
                params.push((k.to_string(), v.to_string()));
            }
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

    /// Create a new list.
    pub async fn create(&self, request: &CreateListRequest) -> Result<List> {
        let req = self.client.request(Method::POST, "/list/v4/lists").await?;
        let req = ConcurClient::with_json(req, request)?;
        self.client.execute(req).await
    }

    /// Update a list.
    pub async fn update(&self, list_id: &str, request: &CreateListRequest) -> Result<List> {
        let req = self
            .client
            .request(Method::PUT, &format!("/list/v4/lists/{}", list_id))
            .await?;
        let req = ConcurClient::with_json(req, request)?;
        self.client.execute(req).await
    }

    /// Delete a list.
    pub async fn delete(&self, list_id: &str) -> Result<()> {
        self.client
            .execute_unit(
                self.client
                    .request(Method::DELETE, &format!("/list/v4/lists/{}", list_id))
                    .await?,
            )
            .await
    }
}

/// List Items API.
pub struct ListItemsApi<'a> {
    client: &'a ConcurClient,
}

impl<'a> ListItemsApi<'a> {
    pub fn new(client: &'a ConcurClient) -> Self {
        Self { client }
    }

    /// Get a list item by ID.
    pub async fn get(&self, item_id: &str) -> Result<ListItem> {
        self.client
            .request(Method::GET, &format!("/list/v4/items/{}", item_id))
            .await?
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// Create a list item.
    pub async fn create(&self, request: &CreateListItemRequest) -> Result<ListItem> {
        let req = self.client.request(Method::POST, "/list/v4/items").await?;
        let req = ConcurClient::with_json(req, request)?;
        self.client.execute(req).await
    }

    /// Update a list item.
    pub async fn update(&self, item_id: &str, request: &CreateListItemRequest) -> Result<ListItem> {
        let req = self
            .client
            .request(Method::PUT, &format!("/list/v4/items/{}", item_id))
            .await?;
        let req = ConcurClient::with_json(req, request)?;
        self.client.execute(req).await
    }

    /// Delete a list item globally.
    pub async fn delete(&self, item_id: &str) -> Result<()> {
        self.client
            .execute_unit(
                self.client
                    .request(Method::DELETE, &format!("/list/v4/items/{}", item_id))
                    .await?,
            )
            .await
    }

    /// Delete a list item from a specific list.
    pub async fn delete_from_list(&self, list_id: &str, item_id: &str) -> Result<()> {
        self.client
            .execute_unit(
                self.client
                    .request(
                        Method::DELETE,
                        &format!("/list/v4/lists/{}/items/{}", list_id, item_id),
                    )
                    .await?,
            )
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    pub id: String,
    pub value: String,
    #[serde(rename = "levelCount")]
    pub level_count: i32,
    #[serde(rename = "searchCriteria")]
    pub search_criteria: String,
    #[serde(rename = "displayFormat")]
    pub display_format: String,
    pub category: ListCategory,
    #[serde(rename = "isReadOnly")]
    pub is_read_only: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "managedBy")]
    pub managed_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCategory {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateListRequest {
    pub value: String,
    #[serde(rename = "searchCriteria")]
    pub search_criteria: String,
    #[serde(rename = "displayFormat")]
    pub display_format: String,
    #[serde(rename = "categoryId", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
    pub id: String,
    pub code: String,
    #[serde(rename = "shortCode")]
    pub short_code: String,
    pub value: String,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    pub level: i32,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    pub lists: Vec<ListItemListRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItemListRef {
    pub id: String,
    #[serde(rename = "hasChildren")]
    pub has_children: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateListItemRequest {
    #[serde(rename = "listId")]
    pub list_id: String,
    #[serde(rename = "shortCode")]
    pub short_code: String,
    pub value: String,
    #[serde(rename = "parentId", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "parentCode", skip_serializing_if = "Option::is_none")]
    pub parent_code: Option<String>,
}
