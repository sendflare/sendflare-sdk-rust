# sendflare-sdk-rust

The SDK for sendflare service written in Rust.

## Requirements

- Rust 1.56.0 or higher

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sendflare-sdk = "1.0.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

```rust
use sendflare_sdk::{SendflareClient, models::SendEmailReq};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SendflareClient::new("your-api-token")?;

    let req = SendEmailReq {
        from: "test@example.com".to_string(),
        to: "to@example.com".to_string(),
        subject: "Hello".to_string(),
        body: "Test email".to_string(),
    };

    let response = client.send_email(&req).await?;
    println!("Email sent successfully: {}", response.success);

    Ok(())
}
```

## Usage Examples

### Send Email

```rust
use sendflare_sdk::{SendflareClient, models::SendEmailReq};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SendflareClient::new("your-api-token")?;

    let req = SendEmailReq {
        from: "sender@example.com".to_string(),
        to: "recipient@example.com".to_string(),
        subject: "Subject Here".to_string(),
        body: "Email body content".to_string(),
    };

    let response = client.send_email(&req).await?;
    if response.success {
        println!("Email sent successfully!");
    }

    Ok(())
}
```

### Get Contact List

```rust
use sendflare_sdk::{SendflareClient, models::ListContactReq};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SendflareClient::new("your-api-token")?;

    let req = ListContactReq {
        app_id: "your-app-id".to_string(),
        page: 1,
        page_size: 10,
    };

    let response = client.get_contact_list(&req).await?;
    println!("Total contacts: {}", response.total_count);

    for contact in response.data {
        println!("Email: {}", contact.email_address);
    }

    Ok(())
}
```

### Save Contact

```rust
use sendflare_sdk::{SendflareClient, models::SaveContactReq};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SendflareClient::new("your-api-token")?;

    let mut data = HashMap::new();
    data.insert("firstName".to_string(), "John".to_string());
    data.insert("lastName".to_string(), "Doe".to_string());
    data.insert("company".to_string(), "Acme Corp".to_string());

    let req = SaveContactReq {
        app_id: "your-app-id".to_string(),
        email_address: "john@example.com".to_string(),
        data: Some(data),
    };

    let response = client.save_contact(&req).await?;
    if response.success {
        println!("Contact saved successfully!");
    }

    Ok(())
}
```

### Delete Contact

```rust
use sendflare_sdk::{SendflareClient, models::DeleteContactReq};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SendflareClient::new("your-api-token")?;

    let req = DeleteContactReq {
        email_address: "john@example.com".to_string(),
        app_id: "your-app-id".to_string(),
    };

    let response = client.delete_contact(&req).await?;
    if response.success {
        println!("Contact deleted successfully!");
    }

    Ok(())
}
```

## API Reference

### SendflareClient

#### new

```rust
pub fn new(token: impl Into<String>) -> Result<Self, Box<dyn std::error::Error>>
```

Create a new Sendflare client instance.

**Parameters:**
- `token` - Your Sendflare API token

**Returns:** `Result<SendflareClient, Box<dyn std::error::Error>>`

#### send_email

```rust
pub async fn send_email(&self, req: &SendEmailReq) -> Result<SendEmailResp, Box<dyn std::error::Error>>
```

Send an email.

**Parameters:**
- `req` - Send email request
  - `from` - Sender email address
  - `to` - Recipient email address
  - `subject` - Email subject
  - `body` - Email body content

**Returns:** `Result<SendEmailResp, Box<dyn std::error::Error>>`

#### get_contact_list

```rust
pub async fn get_contact_list(&self, req: &ListContactReq) -> Result<ListContactResp, Box<dyn std::error::Error>>
```

Get contact list with pagination.

**Parameters:**
- `req` - List contact request
  - `app_id` - Application ID
  - `page` - Page number
  - `page_size` - Items per page

**Returns:** `Result<ListContactResp, Box<dyn std::error::Error>>`

#### save_contact

```rust
pub async fn save_contact(&self, req: &SaveContactReq) -> Result<SaveContactResp, Box<dyn std::error::Error>>
```

Create or update a contact.

**Parameters:**
- `req` - Save contact request
  - `app_id` - Application ID
  - `email_address` - Contact email address
  - `data` - Contact data (Option<HashMap<String, String>>)

**Returns:** `Result<SaveContactResp, Box<dyn std::error::Error>>`

#### delete_contact

```rust
pub async fn delete_contact(&self, req: &DeleteContactReq) -> Result<DeleteContactResp, Box<dyn std::error::Error>>
```

Delete a contact.

**Parameters:**
- `req` - Delete contact request
  - `email_address` - Contact email address
  - `app_id` - Application ID

**Returns:** `Result<DeleteContactResp, Box<dyn std::error::Error>>`

## Model Structs

### Request Models

- `SendEmailReq` - Send email request
- `ListContactReq` - Get contact list request
- `SaveContactReq` - Save contact request
- `DeleteContactReq` - Delete contact request
- `PaginateReq` - Pagination request

### Response Models

- `SendEmailResp` - Send email response
- `ListContactResp` - Get contact list response
- `SaveContactResp` - Save contact response
- `DeleteContactResp` - Delete contact response
- `CommonResponse<T>` - Common response wrapper
- `PaginateResp` - Pagination response
- `ContactItem` - Contact information

## Building from Source

```bash
# Clone the repository
git clone https://github.com/sendflare/sendflare-sdk-rust.git
cd sendflare-sdk-rust

# Build
cargo build

# Run tests
cargo test

# Build documentation
cargo doc --open
```

## Testing

Run tests with:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

## Error Handling

All async methods return `Result<T, Box<dyn std::error::Error>>`. It's recommended to use the `?` operator or match statements:

```rust
// Using ? operator
let response = client.send_email(&req).await?;

// Using match
match client.send_email(&req).await {
    Ok(response) => {
        println!("Success: {}", response.success);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

## Async Runtime

This SDK uses `tokio` as the async runtime. Make sure to use `#[tokio::main]` or create a tokio runtime:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Your code here
    Ok(())
}
```

## Dependencies

- **reqwest** (0.11) - HTTP client with JSON support
- **serde** (1.0) - Serialization framework
- **serde_json** (1.0) - JSON serialization
- **tokio** (1.0) - Async runtime

## Documentation

For more information, visit: [https://docs.sendflare.io](https://docs.sendflare.io)

API documentation: [https://docs.rs/sendflare-sdk](https://docs.rs/sendflare-sdk)

## License

[MIT](./LICENSE)

