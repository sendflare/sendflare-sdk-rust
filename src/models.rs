use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pagination request entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginateReq {
    pub page: i32,
    #[serde(rename = "pageSize")]
    pub page_size: i32,
}

/// Pagination response entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginateResp {
    pub page: i32,
    #[serde(rename = "pageSize")]
    pub page_size: i32,
    #[serde(rename = "totalCount")]
    pub total_count: i64,
}

/// Common response entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonResponse<T> {
    #[serde(rename = "requestId")]
    pub request_id: String,
    pub code: i32,
    pub success: bool,
    pub message: String,
    pub ts: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

/// Send Email request entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailReq {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub body: String,
}

/// Send Email response entity
pub type SendEmailResp = CommonResponse<serde_json::Value>;

/// Get Contact list request entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListContactReq {
    #[serde(rename = "appId")]
    pub app_id: String,
    pub page: i32,
    #[serde(rename = "pageSize")]
    pub page_size: i32,
}

/// Get Contact list response entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListContactResp {
    pub page: i32,
    #[serde(rename = "pageSize")]
    pub page_size: i32,
    #[serde(rename = "totalCount")]
    pub total_count: i64,
    pub list: Vec<HashMap<String, String>>,
}

/// Save contact request entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveContactReq {
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, String>>,
}

/// Save contact response entity
pub type SaveContactResp = CommonResponse<serde_json::Value>;

/// Delete a contact request entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteContactReq {
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[serde(rename = "appId")]
    pub app_id: String,
}

/// Delete contact response entity
pub type DeleteContactResp = CommonResponse<serde_json::Value>;

