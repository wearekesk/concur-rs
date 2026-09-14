//! SAP Concur Travel Request API v4.
//!
//! Covers Request, Workflow, Expected Expenses, and Allocations.

use crate::client::ConcurClient;
use crate::error::Result;
use crate::models::{Link, Money, OwnerRef, StatusRef};
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// Request API.
pub struct RequestApi<'a> {
    client: &'a ConcurClient,
}

impl<'a> RequestApi<'a> {
    pub fn new(client: &'a ConcurClient) -> Self {
        Self { client }
    }

    /// List requests.
    pub async fn list(&self, query: &RequestListQuery) -> Result<RequestList> {
        let mut path = "/travelrequest/v4/requests".to_string();
        let mut params: Vec<(String, String)> = Vec::new();
        if let Some(v) = &query.view {
            params.push(("view".to_string(), v.clone()));
        }
        if let Some(v) = &query.user_id {
            params.push(("userId".to_string(), v.clone()));
        }
        if let Some(v) = query.start {
            params.push(("start".to_string(), v.to_string()));
        }
        if let Some(v) = query.limit {
            params.push(("limit".to_string(), v.to_string()));
        }
        if let Some(v) = &query.approved_before {
            params.push(("approvedBefore".to_string(), v.clone()));
        }
        if let Some(v) = &query.approved_after {
            params.push(("approvedAfter".to_string(), v.clone()));
        }
        if let Some(v) = &query.modified_before {
            params.push(("modifiedBefore".to_string(), v.clone()));
        }
        if let Some(v) = &query.modified_after {
            params.push(("modifiedAfter".to_string(), v.clone()));
        }
        if let Some(v) = &query.sort_field {
            params.push(("sortField".to_string(), v.clone()));
        }
        if let Some(v) = &query.sort_order {
            params.push(("sortOrder".to_string(), v.clone()));
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

    /// Get a request by UUID.
    pub async fn get(&self, request_uuid: &str, user_id: Option<&str>) -> Result<Request> {
        let mut path = format!("/travelrequest/v4/requests/{}", request_uuid);
        if let Some(uid) = user_id {
            path.push_str(&format!("?userId={}", urlencoding::encode(uid)));
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

    /// Create a new request.
    pub async fn create(&self, request: &Request, user_id: Option<&str>) -> Result<Request> {
        let mut path = "/travelrequest/v4/requests".to_string();
        if let Some(uid) = user_id {
            path.push_str(&format!("?userId={}", urlencoding::encode(uid)));
        }
        let req = self.client.request(Method::POST, &path).await?;
        let req = ConcurClient::with_json(req, request)?;
        self.client.execute(req).await
    }

    /// Update a request.
    pub async fn update(
        &self,
        request_uuid: &str,
        request: &Request,
        user_id: Option<&str>,
    ) -> Result<Request> {
        let mut path = format!("/travelrequest/v4/requests/{}", request_uuid);
        if let Some(uid) = user_id {
            path.push_str(&format!("?userId={}", urlencoding::encode(uid)));
        }
        let req = self.client.request(Method::PUT, &path).await?;
        let req = ConcurClient::with_json(req, request)?;
        self.client.execute(req).await
    }

    /// Delete a request.
    pub async fn delete(&self, request_uuid: &str, user_id: Option<&str>) -> Result<()> {
        let mut path = format!("/travelrequest/v4/requests/{}", request_uuid);
        if let Some(uid) = user_id {
            path.push_str(&format!("?userId={}", urlencoding::encode(uid)));
        }
        self.client
            .execute_unit(self.client.request(Method::DELETE, &path).await?)
            .await
    }
}

/// Request Workflow API.
pub struct RequestWorkflowApi<'a> {
    client: &'a ConcurClient,
}

impl<'a> RequestWorkflowApi<'a> {
    pub fn new(client: &'a ConcurClient) -> Self {
        Self { client }
    }

    /// Perform a workflow action: submit, approve, recall, sendback, cancel, close, reopen.
    pub async fn action(
        &self,
        request_uuid: &str,
        action: &str,
        comment: Option<&str>,
        user_id: Option<&str>,
        company_id: Option<&str>,
    ) -> Result<Request> {
        let mut path = format!("/travelrequest/v4/requests/{}/{}", request_uuid, action);
        let mut params: Vec<(String, String)> = Vec::new();
        if let Some(c) = comment {
            params.push(("comment".to_string(), c.to_string()));
        }
        if let Some(uid) = user_id {
            params.push(("userId".to_string(), uid.to_string()));
        }
        if let Some(cid) = company_id {
            params.push(("companyID".to_string(), cid.to_string()));
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
        let body = comment.map(|c| serde_json::json!({ "comment": c }));
        let req = self.client.request(Method::POST, &path).await?;
        let req = match body {
            Some(b) => ConcurClient::with_json(req, &b)?,
            None => req,
        };
        self.client.execute(req).await
    }
}

/// Request Expected Expenses API.
pub struct ExpectedExpensesApi<'a> {
    client: &'a ConcurClient,
}

impl<'a> ExpectedExpensesApi<'a> {
    pub fn new(client: &'a ConcurClient) -> Self {
        Self { client }
    }

    /// List expected expenses for a request.
    pub async fn list(
        &self,
        request_uuid: &str,
        user_id: Option<&str>,
    ) -> Result<Vec<ExpectedExpense>> {
        let mut path = format!("/travelrequest/v4/requests/{}/expenses", request_uuid);
        if let Some(uid) = user_id {
            path.push_str(&format!("?userId={}", urlencoding::encode(uid)));
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

    /// Create an expected expense.
    pub async fn create(
        &self,
        request_uuid: &str,
        expense: &ExpectedExpense,
        user_id: Option<&str>,
    ) -> Result<ExpectedExpense> {
        let mut path = format!("/travelrequest/v4/requests/{}/expenses", request_uuid);
        if let Some(uid) = user_id {
            path.push_str(&format!("?userId={}", urlencoding::encode(uid)));
        }
        let req = self.client.request(Method::POST, &path).await?;
        let req = ConcurClient::with_json(req, expense)?;
        self.client.execute(req).await
    }

    /// Get an expected expense.
    pub async fn get(&self, expense_uuid: &str, user_id: Option<&str>) -> Result<ExpectedExpense> {
        let mut path = format!("/travelrequest/v4/expenses/{}", expense_uuid);
        if let Some(uid) = user_id {
            path.push_str(&format!("?userId={}", urlencoding::encode(uid)));
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

    /// Update an expected expense.
    pub async fn update(
        &self,
        expense_uuid: &str,
        expense: &ExpectedExpense,
        user_id: Option<&str>,
    ) -> Result<ExpectedExpense> {
        let mut path = format!("/travelrequest/v4/expenses/{}", expense_uuid);
        if let Some(uid) = user_id {
            path.push_str(&format!("?userId={}", urlencoding::encode(uid)));
        }
        let req = self.client.request(Method::PUT, &path).await?;
        let req = ConcurClient::with_json(req, expense)?;
        self.client.execute(req).await
    }

    /// Delete an expected expense.
    pub async fn delete(&self, expense_uuid: &str, user_id: Option<&str>) -> Result<()> {
        let mut path = format!("/travelrequest/v4/expenses/{}", expense_uuid);
        if let Some(uid) = user_id {
            path.push_str(&format!("?userId={}", urlencoding::encode(uid)));
        }
        self.client
            .execute_unit(self.client.request(Method::DELETE, &path).await?)
            .await
    }
}

/// Request Allocations API.
pub struct RequestAllocationsApi<'a> {
    client: &'a ConcurClient,
}

impl<'a> RequestAllocationsApi<'a> {
    pub fn new(client: &'a ConcurClient) -> Self {
        Self { client }
    }

    /// Get an allocation.
    pub async fn get(
        &self,
        request_uuid: &str,
        allocation_uuid: &str,
        user_id: Option<&str>,
    ) -> Result<Vec<RequestAllocation>> {
        let mut path = format!(
            "/travelrequest/v4/requests/{}/allocations/{}",
            request_uuid, allocation_uuid
        );
        if let Some(uid) = user_id {
            path.push_str(&format!("?userId={}", urlencoding::encode(uid)));
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

    /// Update an allocation.
    pub async fn update(
        &self,
        request_uuid: &str,
        allocation_uuid: &str,
        allocation: &RequestAllocation,
        user_id: Option<&str>,
    ) -> Result<Vec<RequestAllocation>> {
        let mut path = format!(
            "/travelrequest/v4/requests/{}/allocations/{}",
            request_uuid, allocation_uuid
        );
        if let Some(uid) = user_id {
            path.push_str(&format!("?userId={}", urlencoding::encode(uid)));
        }
        let req = self.client.request(Method::PUT, &path).await?;
        let req = ConcurClient::with_json(req, allocation)?;
        self.client.execute(req).await
    }
}

#[derive(Debug, Clone, Default)]
pub struct RequestListQuery {
    pub view: Option<String>,
    pub user_id: Option<String>,
    pub start: Option<u32>,
    pub limit: Option<u32>,
    pub approved_before: Option<String>,
    pub approved_after: Option<String>,
    pub modified_before: Option<String>,
    pub modified_after: Option<String>,
    pub sort_field: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RequestList {
    pub data: Vec<Request>,
    pub operations: Option<Vec<Link>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Request {
    pub id: Option<String>,
    pub href: Option<String>,
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "businessPurpose")]
    pub business_purpose: Option<String>,
    pub comment: Option<String>,
    #[serde(rename = "approvalStatus")]
    pub approval_status: Option<StatusRef>,
    pub approved: Option<bool>,
    #[serde(rename = "pendingApproval")]
    pub pending_approval: Option<bool>,
    #[serde(rename = "canceledPostApproval")]
    pub canceled_post_approval: Option<bool>,
    pub closed: Option<bool>,
    pub owner: Option<OwnerRef>,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    #[serde(rename = "creationDate")]
    pub creation_date: Option<String>,
    #[serde(rename = "lastModified")]
    pub last_modified: Option<String>,
    #[serde(rename = "submitDate")]
    pub submit_date: Option<String>,
    #[serde(rename = "totalApprovedAmount")]
    pub total_approved_amount: Option<Money>,
    #[serde(rename = "totalPostedAmount")]
    pub total_posted_amount: Option<Money>,
    #[serde(rename = "totalRemainingAmount")]
    pub total_remaining_amount: Option<Money>,
    #[serde(rename = "mainDestination")]
    pub main_destination: Option<serde_json::Value>,
    pub policy: Option<serde_json::Value>,
    #[serde(rename = "travelAgency")]
    pub travel_agency: Option<serde_json::Value>,
    pub expenses: Option<Vec<serde_json::Value>>,
    pub operations: Option<Vec<Link>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExpectedExpense {
    pub id: Option<String>,
    pub href: Option<String>,
    #[serde(rename = "expenseType")]
    pub expense_type: Option<serde_json::Value>,
    #[serde(rename = "transactionDate")]
    pub transaction_date: Option<String>,
    #[serde(rename = "transactionAmount")]
    pub transaction_amount: Option<Money>,
    #[serde(rename = "postedAmount")]
    pub posted_amount: Option<Money>,
    #[serde(rename = "approvedAmount")]
    pub approved_amount: Option<Money>,
    #[serde(rename = "remainingAmount")]
    pub remaining_amount: Option<Money>,
    #[serde(rename = "businessPurpose")]
    pub business_purpose: Option<String>,
    pub location: Option<serde_json::Value>,
    #[serde(rename = "tripData")]
    pub trip_data: Option<serde_json::Value>,
    pub allocations: Option<Vec<RequestAllocation>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequestAllocation {
    #[serde(rename = "allocationId")]
    pub allocation_id: Option<String>,
    #[serde(rename = "expenseId")]
    pub expense_id: Option<String>,
    #[serde(rename = "allocationAmount")]
    pub allocation_amount: Option<Money>,
    #[serde(rename = "approvedAmount")]
    pub approved_amount: Option<Money>,
    #[serde(rename = "postedAmount")]
    pub posted_amount: Option<Money>,
    pub percentage: Option<i32>,
    #[serde(rename = "percentEdited")]
    pub percent_edited: Option<bool>,
    #[serde(rename = "systemAllocation")]
    pub system_allocation: Option<bool>,
}
