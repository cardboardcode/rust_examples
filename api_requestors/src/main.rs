// Add the following dependencies to your Cargo.toml file:
// reqwest = { version = "0.11", features = ["json"] }
// tokio = { version = "1", features = ["full"] }

use reqwest::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Define the API URL
    let url = "https://catfact.ninja/fact";

    // Create a client
    let client = Client::new();

    // Add headers to the request
    let response = client
        .get(url)
        .header("Accept", "application/json")
        .header("User-Agent", "Rust-reqwest-client/0.11")
        .send()
        .await?;

    // Check the status code
    if response.status().is_success() {
        // Parse the response body as text
        let body = response.text().await?;
        println!("Cat Fact: \n{}", body);
    } else {
        println!("Failed to fetch data. Status: {}", response.status());
    }

    Ok(())
}
