use crate::models::*;
use reqwest::{Client, StatusCode};
use std::time::Duration;

const BASE_URL: &str = "https://api.sendflare.io";
const REQUEST_TIMEOUT: u64 = 10; // seconds

/// Sendflare SDK Client
#[derive(Clone)]
pub struct SendflareClient {
    token: String,
    client: Client,
}

impl SendflareClient {
    /// Create a new Sendflare client instance
    ///
    /// # Arguments
    ///
    /// * `token` - API token
    pub fn new(token: impl Into<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT))
            .build()?;

        Ok(Self {
            token: token.into(),
            client,
        })
    }

    /// Send an email
    ///
    /// # Arguments
    ///
    /// * `req` - Send email request
    pub async fn send_email(&self, req: &SendEmailReq) -> Result<SendEmailResp, Box<dyn std::error::Error>> {
        let path = "/v1/send";
        let url = format!("{}{}", BASE_URL, path);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .json(req)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Get contact list
    ///
    /// # Arguments
    ///
    /// * `req` - List contact request
    pub async fn get_contact_list(&self, req: &ListContactReq) -> Result<ListContactResp, Box<dyn std::error::Error>> {
        let path = "/v1/contact";
        let url = format!("{}{}", BASE_URL, path);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .query(&[
                ("appId", &req.app_id),
                ("page", &req.page.to_string()),
                ("pageSize", &req.page_size.to_string()),
            ])
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Create or update contact
    ///
    /// # Arguments
    ///
    /// * `req` - Save contact request
    pub async fn save_contact(&self, req: &SaveContactReq) -> Result<SaveContactResp, Box<dyn std::error::Error>> {
        let path = "/v1/contact";
        let url = format!("{}{}", BASE_URL, path);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .json(req)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Delete a contact
    ///
    /// # Arguments
    ///
    /// * `req` - Delete contact request
    pub async fn delete_contact(&self, req: &DeleteContactReq) -> Result<DeleteContactResp, Box<dyn std::error::Error>> {
        let path = "/v1/contact";
        let url = format!("{}{}", BASE_URL, path);

        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .query(&[
                ("appId", &req.app_id),
                ("emailAddress", &req.email_address),
            ])
            .send()
            .await?;

        self.handle_response(response).await
    }

    async fn handle_response<T>(&self, response: reqwest::Response) -> Result<T, Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = response.status();
        
        if !status.is_success() {
            return Err(format!("HTTP error: {}", status).into());
        }

        let body = response.text().await?;
        let result: T = serde_json::from_str(&body)?;
        Ok(result)
    }
}

