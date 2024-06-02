// Import for constructing JSON objects in Rust code
use serde_json::json;

// Marks the following function as an asynchronous test
#[tokio::test]
async fn test_contract_is_operational() -> Result<(), Box<dyn std::error::Error>> {
  // Create a NEAR workspace sandbox for testing
  let sandbox = near_workspaces::sandbox().await?;

  // Compile the current project's code into a WASM module
  let contract_wasm = near_workspaces::compile_project("./").await?;

  // Deploy the compiled WASM code to the sandbox (creating a contract instance)
  let contract = sandbox.dev_deploy(&contract_wasm).await?;

  // Create a simulated user account within the sandbox
  let user_account = sandbox.dev_create_account().await?;

  // **Call the contract's set_greeting method:**
  // - Specify the contract ID and method name
  // - Construct a JSON object with the new greeting message as an argument
  // - Execute the call asynchronously and wait for the result
  // - Check if the call was successful
  let outcome = user_account
    .call(contract.id(), "set_greeting")
    .args_json(json!({"greeting": "Hello, Near!"}))
    .transact()
    .await?;
  assert!(outcome.is_success());

  // **View the current greeting stored in the contract:**
  // - Specify the contract ID and method name (get_greeting)
  // - Send an empty JSON object (no arguments needed)
  // - Execute the view call asynchronously and wait for the result
  // - Deserialize the result from JSON to a String
  // - Assert that the retrieved greeting matches the expected value
  let user_message_outcome = contract
    .view("get_greeting")
    .args_json(json!({}))
    .await?;
  assert_eq!(user_message_outcome.json::<String>()?, "Hello, Near!");

  Ok(())
}