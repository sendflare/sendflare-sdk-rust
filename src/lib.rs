//! Sendflare SDK for Rust
//!
//! This is the official Sendflare SDK for Rust, providing an easy way to interact
//! with the Sendflare API.
//!
//! # Examples
//!
//! ```no_run
//! use sendflare_sdk::{SendflareClient, models::SendEmailReq};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = SendflareClient::new("your-api-token")?;
//!
//!     let req = SendEmailReq {
//!         from: "test@example.com".to_string(),
//!         to: "to@example.com".to_string(),
//!         subject: "Hello".to_string(),
//!         body: "Test email".to_string(),
//!     };
//!
//!     let response = client.send_email(&req).await?;
//!     println!("Email sent: {}", response.success);
//!
//!     Ok(())
//! }
//! ```

pub mod models;
pub mod client;

pub use client::SendflareClient;
pub use models::*;

