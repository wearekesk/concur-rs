//! SAP Concur Expense APIs v4.
//!
//! Covers Expense Reports, Expenses (entries), and Workflow.

use crate::client::ConcurClient;
use crate::error::Result;
use crate::models::{
    CustomData, ExchangeRate, ExpenseType, Link, Location, Money, PaymentType, Vendor,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// Expense Reports API.
pub struct ReportsApi<'a> {
    client: &'a ConcurClient,
}

impl<'a> ReportsApi<'a> {
    pub fn new(client: &'a ConcurClient) -> Self {
        Self { client }
    }

    fn base(&self, user_id: &str, context: &str) -> String {
        format!(
            "/expensereports/v4/users/{}/context/{}/reports",
            user_id, context
        )
    }

    /// Retrieve a report by ID.
    pub async fn get(&self, user_id: &str, context: &str, report_id: &str) -> Result<Report> {
        self.client
            .request(
                Method::GET,
                &format!("{}/{}", self.base(user_id, context), report_id),
            )
            .await?
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// Create a new report.
    pub async fn create(
        &self,
        user_id: &str,
        context: &str,
        report: &CreateReport,
    ) -> Result<CreateReportResponse> {
        let req = self
            .client
            .request(Method::POST, &self.base(user_id, context))
            .await?;
        let req = ConcurClient::with_json(req, report)?;
        self.client.execute(req).await
    }

    /// Update an unsubmitted report (JSON Merge Patch).
    pub async fn update(
        &self,
        user_id: &str,
        context: &str,
        report_id: &str,
        patch: &CreateReport,
    ) -> Result<()> {
        let req = self
            .client
            .request(
                Method::PATCH,
                &format!("{}/{}", self.base(user_id, context), report_id),
            )
            .await?;
        let req = ConcurClient::with_json(req, patch)?;
        self.client.execute_unit(req).await
    }

    /// Delete an unsubmitted report.
    pub async fn delete(&self, user_id: &str, context: &str, report_id: &str) -> Result<()> {
        self.client
            .execute_unit(
                self.client
                    .request(
                        Method::DELETE,
                        &format!("{}/{}", self.base(user_id, context), report_id),
                    )
                    .await?,
            )
            .await
    }

    /// Get configured form fields for a report.
    pub async fn form_fields(
        &self,
        report_id: &str,
        policy_id: Option<&str>,
    ) -> Result<Vec<FormField>> {
        let mut path = format!("/expensereports/v4/reports/{}/formFields", report_id);
        if let Some(pid) = policy_id {
            path.push_str(&format!("?policyId={}", urlencoding::encode(pid)));
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

/// Expenses (entries) API.
pub struct ExpensesApi<'a> {
    client: &'a ConcurClient,
}

impl<'a> ExpensesApi<'a> {
    pub fn new(client: &'a ConcurClient) -> Self {
        Self { client }
    }

    fn base(&self, user_id: &str, context: &str, report_id: &str) -> String {
        format!(
            "/expensereports/v4/users/{}/context/{}/reports/{}/expenses",
            user_id, context, report_id
        )
    }

    /// List expenses on a report.
    pub async fn list(
        &self,
        user_id: &str,
        context: &str,
        report_id: &str,
    ) -> Result<Vec<Expense>> {
        self.client
            .request(Method::GET, &self.base(user_id, context, report_id))
            .await?
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// Get a single expense.
    pub async fn get(
        &self,
        user_id: &str,
        context: &str,
        report_id: &str,
        expense_id: &str,
    ) -> Result<Expense> {
        self.client
            .request(
                Method::GET,
                &format!("{}/{}", self.base(user_id, context, report_id), expense_id),
            )
            .await?
            .send()
            .await?
            .json()
            .await
            .map_err(Into::into)
    }

    /// Create an expense on a report.
    pub async fn create(
        &self,
        user_id: &str,
        context: &str,
        report_id: &str,
        expense: &CreateExpense,
    ) -> Result<CreateExpenseResponse> {
        let req = self
            .client
            .request(Method::POST, &self.base(user_id, context, report_id))
            .await?;
        let req = ConcurClient::with_json(req, expense)?;
        self.client.execute(req).await
    }

    /// Update an expense (JSON Merge Patch).
    pub async fn update(
        &self,
        user_id: &str,
        context: &str,
        report_id: &str,
        expense_id: &str,
        patch: &CreateExpense,
    ) -> Result<()> {
        let req = self
            .client
            .request(
                Method::PATCH,
                &format!("{}/{}", self.base(user_id, context, report_id), expense_id),
            )
            .await?;
        let req = ConcurClient::with_json(req, patch)?;
        self.client.execute_unit(req).await
    }

    /// Delete an expense.
    pub async fn delete(
        &self,
        user_id: &str,
        context: &str,
        report_id: &str,
        expense_id: &str,
    ) -> Result<()> {
        self.client
            .execute_unit(
                self.client
                    .request(
                        Method::DELETE,
                        &format!("{}/{}", self.base(user_id, context, report_id), expense_id),
                    )
                    .await?,
            )
            .await
    }

    /// List itemizations for an expense.
    pub async fn itemizations(
        &self,
        user_id: &str,
        context: &str,
        report_id: &str,
        expense_id: &str,
    ) -> Result<Vec<serde_json::Value>> {
        self.client
            .request(
                Method::GET,
                &format!(
                    "{}/{}/itemizations",
                    self.base(user_id, context, report_id),
                    expense_id
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

/// Expense Workflow API.
pub struct WorkflowApi<'a> {
    client: &'a ConcurClient,
}

impl<'a> WorkflowApi<'a> {
    pub fn new(client: &'a ConcurClient) -> Self {
        Self { client }
    }

    /// Approve, send back, or recall a report.
    pub async fn action(
        &self,
        report_id: &str,
        action: ReportAction,
        request: &WorkflowRequest,
        user_id: Option<&str>,
        context: Option<&str>,
    ) -> Result<()> {
        let path = match (user_id, context) {
            (Some(uid), Some(ctx)) => format!(
                "/expensereports/v4/users/{}/context/{}/reports/{}/{}",
                uid,
                ctx,
                report_id,
                action.path()
            ),
            _ => format!("/expensereports/v4/reports/{}/{}", report_id, action.path()),
        };
        let req = self.client.request(Method::PATCH, &path).await?;
        let req = ConcurClient::with_json(req, request)?;
        self.client.execute_unit(req).await
    }

    /// Submit a report.
    pub async fn submit(&self, user_id: &str, report_id: &str) -> Result<()> {
        self.client
            .execute_unit(
                self.client
                    .request(
                        Method::PATCH,
                        &format!(
                            "/expensereports/v4/users/{}/reports/{}/submit",
                            user_id, report_id
                        ),
                    )
                    .await?,
            )
            .await
    }

    /// Get cost objects for a report.
    pub async fn cost_objects(&self, user_id: &str, report_id: &str) -> Result<Vec<CostObject>> {
        self.client
            .request(
                Method::GET,
                &format!(
                    "/expensereports/v4/users/{}/reports/{}/costObjectsForApprover",
                    user_id, report_id
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

#[derive(Debug, Clone, Copy)]
pub enum ReportAction {
    Approve,
    SendBack,
    Recall,
}

impl ReportAction {
    fn path(&self) -> &'static str {
        match self {
            ReportAction::Approve => "approve",
            ReportAction::SendBack => "sendBack",
            ReportAction::Recall => "recall",
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(
        rename = "expenseRejectedComment",
        skip_serializing_if = "Option::is_none"
    )]
    pub expense_rejected_comment: Option<String>,
    #[serde(rename = "expectedStepCode", skip_serializing_if = "Option::is_none")]
    pub expected_step_code: Option<String>,
    #[serde(
        rename = "expectedStepSequence",
        skip_serializing_if = "Option::is_none"
    )]
    pub expected_step_sequence: Option<String>,
    #[serde(rename = "statusId", skip_serializing_if = "Option::is_none")]
    pub status_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    #[serde(rename = "reportId")]
    pub report_id: String,
    pub name: Option<String>,
    #[serde(rename = "businessPurpose")]
    pub business_purpose: Option<String>,
    #[serde(rename = "approvalStatus")]
    pub approval_status: Option<String>,
    #[serde(rename = "approvalStatusId")]
    pub approval_status_id: Option<String>,
    #[serde(rename = "paymentStatus")]
    pub payment_status: Option<String>,
    #[serde(rename = "paymentStatusId")]
    pub payment_status_id: Option<String>,
    #[serde(rename = "reportDate")]
    pub report_date: Option<String>,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    #[serde(rename = "policyId")]
    pub policy_id: Option<String>,
    #[serde(rename = "policy")]
    pub policy_name: Option<String>,
    #[serde(rename = "countryCode")]
    pub country_code: Option<String>,
    #[serde(rename = "countrySubDivisionCode")]
    pub country_sub_division_code: Option<String>,
    #[serde(rename = "reportTotal")]
    pub report_total: Option<Money>,
    #[serde(rename = "claimedAmount")]
    pub claimed_amount: Option<Money>,
    #[serde(rename = "approvedAmount")]
    pub approved_amount: Option<Money>,
    #[serde(rename = "customData")]
    pub custom_data: Option<Vec<CustomData>>,
    pub links: Option<Vec<Link>>,
    #[serde(rename = "creationDate")]
    pub creation_date: Option<String>,
    #[serde(rename = "submitDate")]
    pub submit_date: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateReport {
    pub name: Option<String>,
    #[serde(rename = "businessPurpose")]
    pub business_purpose: Option<String>,
    pub comment: Option<String>,
    #[serde(rename = "countryCode")]
    pub country_code: Option<String>,
    #[serde(rename = "countrySubDivisionCode")]
    pub country_sub_division_code: Option<String>,
    #[serde(rename = "policyId")]
    pub policy_id: Option<String>,
    #[serde(rename = "reportSource")]
    pub report_source: Option<String>,
    #[serde(rename = "customData")]
    pub custom_data: Option<Vec<CustomData>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateReportResponse {
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    #[serde(rename = "expenseId")]
    pub expense_id: String,
    #[serde(rename = "businessPurpose")]
    pub business_purpose: Option<String>,
    #[serde(rename = "transactionDate")]
    pub transaction_date: Option<String>,
    #[serde(rename = "transactionAmount")]
    pub transaction_amount: Option<Money>,
    #[serde(rename = "postedAmount")]
    pub posted_amount: Option<Money>,
    #[serde(rename = "approvedAmount")]
    pub approved_amount: Option<Money>,
    #[serde(rename = "claimedAmount")]
    pub claimed_amount: Option<Money>,
    #[serde(rename = "expenseType")]
    pub expense_type: Option<ExpenseType>,
    #[serde(rename = "paymentType")]
    pub payment_type: Option<PaymentType>,
    pub location: Option<Location>,
    pub vendor: Option<Vendor>,
    #[serde(rename = "receiptImageId")]
    pub receipt_image_id: Option<String>,
    #[serde(rename = "ereceiptImageId")]
    pub ereceipt_image_id: Option<String>,
    #[serde(rename = "exchangeRate")]
    pub exchange_rate: Option<ExchangeRate>,
    #[serde(rename = "attendeeCount")]
    pub attendee_count: Option<i32>,
    #[serde(rename = "isPersonalExpense")]
    pub is_personal_expense: Option<bool>,
    #[serde(rename = "customData")]
    pub custom_data: Option<Vec<CustomData>>,
    pub links: Option<Vec<Link>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateExpense {
    #[serde(rename = "expenseTypeId")]
    pub expense_type_id: Option<String>,
    #[serde(rename = "transactionDate")]
    pub transaction_date: Option<String>,
    #[serde(rename = "transactionAmount")]
    pub transaction_amount: Option<Money>,
    #[serde(rename = "businessPurpose")]
    pub business_purpose: Option<String>,
    #[serde(rename = "paymentTypeId")]
    pub payment_type_id: Option<String>,
    #[serde(rename = "locationId")]
    pub location_id: Option<String>,
    pub vendor: Option<Vendor>,
    pub comment: Option<String>,
    #[serde(rename = "customData")]
    pub custom_data: Option<Vec<CustomData>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateExpenseResponse {
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    #[serde(rename = "fieldId")]
    pub field_id: String,
    #[serde(rename = "fieldName")]
    pub field_name: Option<String>,
    #[serde(rename = "controlType")]
    pub control_type: Option<String>,
    #[serde(rename = "dataType")]
    pub data_type: Option<String>,
    #[serde(rename = "fieldAccess")]
    pub field_access: Option<String>,
    #[serde(rename = "isRequired")]
    pub is_required: Option<bool>,
    #[serde(rename = "listId")]
    pub list_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostObject {
    pub name: String,
    #[serde(rename = "approvedAmount")]
    pub approved_amount: Money,
    #[serde(rename = "claimedAmount")]
    pub claimed_amount: Money,
    #[serde(rename = "approverId")]
    pub approver_id: String,
    pub expenses: Vec<CostObjectExpense>,
    #[serde(rename = "isOwnedByCaller")]
    pub is_owned_by_caller: bool,
    #[serde(rename = "isFullyApproved")]
    pub is_fully_approved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostObjectExpense {
    #[serde(rename = "approvedAmount")]
    pub approved_amount: Money,
    #[serde(rename = "postedAmount")]
    pub posted_amount: Money,
    pub id: String,
}
