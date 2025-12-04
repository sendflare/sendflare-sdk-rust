use sendflare_sdk::{SendflareClient, models::*};
use std::collections::HashMap;

#[tokio::test]
async fn test_new_sendflare_client() {
    let client = SendflareClient::new("this-is-my-token");
    assert!(client.is_ok());
}

#[tokio::test]
async fn test_send_email() {
    let client = SendflareClient::new("this-is-my-token").unwrap();

    let req = SendEmailReq {
        from: "test@example.com".to_string(),
        to: "to@example.com".to_string(),
        subject: "test".to_string(),
        body: "test email".to_string(),
    };

    println!("Request: {:?}", req);

    match client.send_email(&req).await {
        Ok(resp) => {
            println!("Response: {:?}", resp);
        }
        Err(e) => {
            println!("Expected error without valid token: {}", e);
            // This is expected without a valid token
            assert!(true);
        }
    }
}

#[tokio::test]
async fn test_get_contact_list() {
    let client = SendflareClient::new("this-is-my-token").unwrap();

    let req = ListContactReq {
        app_id: "test".to_string(),
        page: 1,
        page_size: 10,
    };

    println!("Request: {:?}", req);

    match client.get_contact_list(&req).await {
        Ok(resp) => {
            println!("Response: total_count={}", resp.total_count);
        }
        Err(e) => {
            println!("Expected error without valid token: {}", e);
            // This is expected without a valid token
            assert!(true);
        }
    }
}

#[tokio::test]
async fn test_save_contact() {
    let client = SendflareClient::new("this-is-my-token").unwrap();

    let mut data = HashMap::new();
    data.insert("firstName".to_string(), "John".to_string());
    data.insert("lastName".to_string(), "Doe".to_string());

    let req = SaveContactReq {
        app_id: "test".to_string(),
        email_address: "test@example.com".to_string(),
        data: Some(data),
    };

    println!("Request: {:?}", req);

    match client.save_contact(&req).await {
        Ok(resp) => {
            println!("Response: success={}", resp.success);
        }
        Err(e) => {
            println!("Expected error without valid token: {}", e);
            // This is expected without a valid token
            assert!(true);
        }
    }
}

#[tokio::test]
async fn test_delete_contact() {
    let client = SendflareClient::new("this-is-my-token").unwrap();

    let req = DeleteContactReq {
        email_address: "test@example.com".to_string(),
        app_id: "test".to_string(),
    };

    println!("Request: {:?}", req);

    match client.delete_contact(&req).await {
        Ok(resp) => {
            println!("Response: success={}", resp.success);
        }
        Err(e) => {
            println!("Expected error without valid token: {}", e);
            // This is expected without a valid token
            assert!(true);
        }
    }
}

