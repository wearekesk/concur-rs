//! Shared data models used across SAP Concur APIs.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A monetary amount with currency.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Money {
    pub value: f64,
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
}

/// A monetary amount with currency (alternate naming used by some APIs).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Amount {
    pub amount: Option<String>,
    pub currency: Option<String>,
}

/// A generic link relation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Link {
    pub rel: String,
    pub href: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "isTemplated", skip_serializing_if = "Option::is_none")]
    pub is_templated: Option<bool>,
}

/// Pagination metadata returned by list endpoints.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Page {
    pub size: u32,
    #[serde(rename = "totalElements")]
    pub total_elements: u64,
    #[serde(rename = "totalPages")]
    pub total_pages: u32,
    pub number: u32,
}

/// A named reference (id + name).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NamedRef {
    pub id: String,
    pub name: String,
}

/// A generic paged response wrapper.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PagedResponse<T> {
    pub links: Vec<Link>,
    pub content: Vec<T>,
    pub page: Page,
}

/// A custom data field value.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomData {
    pub id: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_item_url: Option<String>,
}

/// Location reference.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Location {
    pub id: String,
    pub name: String,
    pub city: Option<String>,
    #[serde(rename = "countrySubDivisionCode")]
    pub country_sub_division_code: Option<String>,
    #[serde(rename = "countryCode")]
    pub country_code: Option<String>,
}

/// Expense type reference.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExpenseType {
    pub id: String,
    pub name: String,
    pub code: Option<String>,
    #[serde(rename = "isDeleted")]
    pub is_deleted: Option<bool>,
}

/// Payment type reference.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentType {
    pub id: String,
    pub name: String,
    pub code: String,
}

/// Vendor reference.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Vendor {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}

/// Status reference with code and name.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatusRef {
    pub code: String,
    pub name: String,
}

/// Owner / user reference.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OwnerRef {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

/// Exchange rate information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExchangeRate {
    pub value: f64,
    pub operation: String,
}

/// A SCIM-style user name.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct UserName {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
}

/// A SCIM email entry.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Email {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications: Option<bool>,
}

/// A SCIM address entry.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

/// A SCIM meta object.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Meta {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub created: Option<DateTime<Utc>>,
    #[serde(rename = "lastModified")]
    pub last_modified: Option<DateTime<Utc>>,
    pub version: Option<i64>,
    pub location: Option<String>,
}
