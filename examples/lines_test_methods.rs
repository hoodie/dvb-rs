//! Test example to verify which HTTP method works for the Lines API
//!
//! According to the VVO WebAPI specification, the endpoint should use POST:
//! https://raw.githubusercontent.com/kiliankoe/vvo/refs/heads/main/documentation/webapi.md
//!
//! However, the current implementation uses GET, and we need to verify which actually works.
//!
//! Usage:
//!   cargo run --example lines_test_methods [stop_name]
//!
//! Example:
//!   cargo run --example lines_test_methods "Hauptbahnhof"

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LinesParamsPost<'a> {
    stopid: &'a str,
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
    lines: Vec<Value>,
}

const LINES_URL: &str = "https://webapi.vvo-online.de/stt/lines";

async fn test_get_method(stop_id: &str) -> Result<ApiResponse, Box<dyn std::error::Error>> {
    println!("\n=== Testing GET Method ===");
    println!("URL: {}", LINES_URL);
    println!("Method: GET with query parameters");
    println!("Query: stopid={}&format=json", stop_id);

    let response = reqwest::Client::new()
        .get(LINES_URL)
        .query(&[("stopid", stop_id), ("format", "json")])
        .send()
        .await?;

    println!("Status Code: {}", response.status());

    let text = response.text().await?;
    println!(
        "Response (first 500 chars): {}",
        &text[..text.len().min(500)]
    );

    let result: ApiResponse = serde_json::from_str(&text)?;
    Ok(result)
}

async fn test_post_method(stop_id: &str) -> Result<ApiResponse, Box<dyn std::error::Error>> {
    println!("\n=== Testing POST Method ===");
    println!("URL: {}", LINES_URL);
    println!("Method: POST with JSON body");

    let params = LinesParamsPost { stopid: stop_id };
    println!("Body: {}", serde_json::to_string_pretty(&params)?);

    let response = reqwest::Client::new()
        .post(LINES_URL)
        .json(&params)
        .send()
        .await?;

    println!("Status Code: {}", response.status());

    let text = response.text().await?;
    println!(
        "Response (first 500 chars): {}",
        &text[..text.len().min(500)]
    );

    let result: ApiResponse = serde_json::from_str(&text)?;
    Ok(result)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get test stop
    let stop_query = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "Hauptbahnhof".to_string());

    println!("Searching for stop...");
    let stop_result = dvb::find_stops(&stop_query).await?;
    let stop = stop_result.points.first().expect("Stop not found");

    println!("\nStop: {} (ID: {})", stop.name, stop.id);

    // Test GET method (current implementation)
    match test_get_method(&stop.id).await {
        Ok(response) => {
            println!("\n✅ GET Method Response:");
            println!("  Status Code: {}", response.status.code);
            if let Some(msg) = &response.status.message {
                println!("  Message: {}", msg);
            }
            println!("  Number of lines: {}", response.lines.len());

            if response.status.code == "Ok" && !response.lines.is_empty() {
                println!("\n  ✅ GET METHOD WORKS!");
                println!("  Lines found:");
                for (i, line) in response.lines.iter().take(5).enumerate() {
                    if let Some((name, mot)) = Option::zip(line.get("Name"), line.get("Mot")) {
                        println!("    {}. {} ({})", i + 1, name, mot);
                    }
                }
                if response.lines.len() > 5 {
                    println!("    ... and {} more", response.lines.len() - 5);
                }
            } else {
                println!("\n  ❌ GET METHOD FAILED: {}", response.status.code);
            }
        }
        Err(e) => {
            println!("\n❌ GET Method Error: {}", e);
        }
    }

    // Test POST method (specification says to use this)
    match test_post_method(&stop.id).await {
        Ok(response) => {
            println!("\n✅ POST Method Response:");
            println!("  Status Code: {}", response.status.code);
            if let Some(msg) = &response.status.message {
                println!("  Message: {}", msg);
            }
            println!("  Number of lines: {}", response.lines.len());

            if response.status.code == "Ok" && !response.lines.is_empty() {
                println!("\n  ✅ POST METHOD WORKS!");
                println!("  Lines found:");
                for (i, line) in response.lines.iter().take(5).enumerate() {
                    if let Some((name, mot)) = Option::zip(line.get("Name"), line.get("Mot")) {
                        println!("    {}. {} ({})", i + 1, name, mot);
                    }
                }
                if response.lines.len() > 5 {
                    println!("    ... and {} more", response.lines.len() - 5);
                }
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
    println!("  - Current implementation in src/lines.rs is CORRECT");
    println!(
        "  - Specification at line 'POST https://webapi.vvo-online.de/stt/lines' is INCORRECT"
    );
    println!("  - Should be documented as: GET with query parameters");
    println!("\nIf POST works:");
    println!("  - Specification is CORRECT");
    println!("  - src/lines.rs needs to be updated:");
    println!("    - Change .get(LINES_URL) to .post(LINES_URL)");
    println!(
        "    - Change .query(&[(\"format\", \"json\"), (\"stopid\", stop_id)]) to .json(&Params {{ stopid: stop_id }})"
    );
    println!("    - Remove format parameter (JSON is default)");
    println!("\nIf both work:");
    println!("  - API accepts both methods (lenient implementation)");
    println!("  - Recommend following specification (POST) for consistency");
    println!("  - Update src/lines.rs to use POST method");
    println!("\nIf only GET works:");
    println!("  - Keep current implementation");
    println!("  - Document in webapi.md that specification is incorrect");
    println!("  - Submit issue/PR to kiliankoe/vvo repository");
    println!("{}", "=".repeat(60));

    Ok(())
}
