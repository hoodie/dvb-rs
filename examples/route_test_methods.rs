//! Test example to verify which HTTP method works for the Route Planning API
//!
//! According to the VVO WebAPI specification, the endpoint should use POST:
//! https://raw.githubusercontent.com/kiliankoe/vvo/refs/heads/main/documentation/webapi.md
//!
//! However, the current implementation uses GET, and we need to verify which actually works.
//!
//! Usage:
//!   cargo run --example route_test_methods [origin] [destination]
//!
//! Example:
//!   cargo run --example route_test_methods "Hauptbahnhof" "Albertplatz"


use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RouteParams<'a> {
    origin: &'a str,
    destination: &'a str,
    time: String,
    isarrivaltime: bool,
    shorttermchanges: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ApiStatus {
    code: String,
    message: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ApiResponse {
    status: ApiStatus,
    #[serde(default)]
    routes: Vec<Value>,
}

const ROUTE_URL: &str = "https://webapi.vvo-online.de/tr/trips";

async fn test_get_method(params: &RouteParams<'_>) -> Result<ApiResponse, Box<dyn std::error::Error>> {
    println!("\n=== Testing GET Method ===");
    println!("URL: {}", ROUTE_URL);
    println!("Method: GET with query parameters");
    
    let response = reqwest::Client::new()
        .get(ROUTE_URL)
        .query(&params)
        .send()
        .await?;
    
    println!("Status Code: {}", response.status());
    
    let text = response.text().await?;
    println!("Response (first 500 chars): {}", &text[..text.len().min(500)]);
    
    let result: ApiResponse = serde_json::from_str(&text)?;
    Ok(result)
}

async fn test_post_method(params: &RouteParams<'_>) -> Result<ApiResponse, Box<dyn std::error::Error>> {
    println!("\n=== Testing POST Method ===");
    println!("URL: {}", ROUTE_URL);
    println!("Method: POST with JSON body");
    println!("Body: {}", serde_json::to_string_pretty(&params)?);
    
    let response = reqwest::Client::new()
        .post(ROUTE_URL)
        .json(&params)
        .send()
        .await?;
    
    println!("Status Code: {}", response.status());
    
    let text = response.text().await?;
    println!("Response (first 500 chars): {}", &text[..text.len().min(500)]);
    
    let result: ApiResponse = serde_json::from_str(&text)?;
    Ok(result)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get test stops
    let origin_query = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "Hauptbahnhof".to_string());
    let destination_query = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "Albertplatz".to_string());

    println!("Searching for stops...");
    let origin_result = dvb::find_stops(&origin_query).await?;
    let origin = origin_result
        .points
        .first()
        .expect("Origin stop not found");

    let destination_result = dvb::find_stops(&destination_query).await?;
    let destination = destination_result
        .points
        .first()
        .expect("Destination stop not found");

    println!("\nOrigin: {} (ID: {})", origin.name, origin.id);
    println!("Destination: {} (ID: {})", destination.name, destination.id);

    // Use ISO8601 format for time as shown in the spec
    let now = chrono::Local::now();
    let time_iso = now.to_rfc3339();

    let params = RouteParams {
        origin: &origin.id,
        destination: &destination.id,
        time: time_iso,
        isarrivaltime: false,
        shorttermchanges: true,
    };

    // Test GET method
    match test_get_method(&params).await {
        Ok(response) => {
            println!("\n✅ GET Method Response:");
            println!("  Status Code: {}", response.status.code);
            if let Some(msg) = &response.status.message {
                println!("  Message: {}", msg);
            }
            println!("  Number of routes: {}", response.routes.len());
            
            if response.status.code == "Ok" && !response.routes.is_empty() {
                println!("\n  ✅ GET METHOD WORKS!");
            } else {
                println!("\n  ❌ GET METHOD FAILED: {}", response.status.code);
            }
        }
        Err(e) => {
            println!("\n❌ GET Method Error: {}", e);
        }
    }

    // Test POST method
    match test_post_method(&params).await {
        Ok(response) => {
            println!("\n✅ POST Method Response:");
            println!("  Status Code: {}", response.status.code);
            if let Some(msg) = &response.status.message {
                println!("  Message: {}", msg);
            }
            println!("  Number of routes: {}", response.routes.len());
            
            if response.status.code == "Ok" && !response.routes.is_empty() {
                println!("\n  ✅ POST METHOD WORKS!");
            } else {
                println!("\n  ❌ POST METHOD FAILED: {}", response.status.code);
            }
        }
        Err(e) => {
            println!("\n❌ POST Method Error: {}", e);
        }
    }

    println!("\n{}", "=".repeat(60));
    println!("CONCLUSION:");
    println!("{}", "=".repeat(60));
    println!("\nIf GET works:");
    println!("  - Current implementation in src/route.rs is CORRECT");
    println!("  - Specification at line 'POST https://webapi.vvo-online.de/tr/trips' is INCORRECT");
    println!("  - Should be documented as: GET with query parameters");
    println!("\nIf POST works:");
    println!("  - Specification is CORRECT");
    println!("  - src/route.rs needs to be updated:");
    println!("    - Change .get(ROUTE_URL) to .post(ROUTE_URL)");
    println!("    - Change .query(&params) to .json(&params)");
    println!("\nIf both work:");
    println!("  - API accepts both methods (lenient implementation)");
    println!("  - Recommend following specification (POST) for consistency");
    println!("  - Update src/route.rs to use POST method");
    println!("{}", "=".repeat(60));

    Ok(())
}