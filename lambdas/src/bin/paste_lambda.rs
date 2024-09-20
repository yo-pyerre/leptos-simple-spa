use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};
async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let payload = event.payload;
    let first_name = payload["firstName"].as_str().unwrap_or("world");
    Ok(json!({ "message": format!("Hello, {first_name}!") }))
}

// The entry point that runs the Lambda function code.
// Rust runtime client uses Tokio as an async runtime, so annotate with #[tokio::main].
#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await
}
