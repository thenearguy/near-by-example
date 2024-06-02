// Import for constructing JSON objects in Rust code
use serde_json::json;

// Asynchronous test function to verify contract functionality
#[tokio::test]
async fn test_contract_is_operational() -> Result<(), Box<dyn std::error::Error>> {
  // Set up a NEAR workspace sandbox for testing
  let sandbox = near_workspaces::sandbox().await?;

  // Compile the current project's code into a WASM module
  let contract_wasm = near_workspaces::compile_project("./").await?;

  // Deploy the compiled WASM code to the sandbox (creating a contract instance)
  let contract = sandbox.dev_deploy(&contract_wasm).await?;

  // Create a simulated user account within the sandbox
  let user_account = sandbox.dev_create_account().await?;

  // **Test Initial Count:**
  // - Call the contract's "get_count" view method
  // - Check if the initial count retrieved is 0 (default)
  let user_default_outcome = contract
    .view("get_count")
    .args_json(json!({}))
    .await?;
  assert_eq!(user_default_outcome.json::<u32>()?, 0);

  // **Test Increment:**
  // - Call the contract's "increment" method (increases count)
  // - Check if the call was successful
  let outcome = user_account
    .call(contract.id(), "increment")
    .args_json(json!({}))
    .transact()
    .await?;
  assert!(outcome.is_success());

  // - Call "get_count" again to verify the count is incremented to 1
  let user_outcome_after_increment = contract
    .view("get_count")
    .args_json(json!({}))
    .await?;
  assert_eq!(user_outcome_after_increment.json::<u32>()?, 1);

  // **Test Decrement:**
  // - Similar logic as increment test, calling "decrement" and verifying count
  let outcome = user_account
    .call(contract.id(), "decrement")
    .args_json(json!({}))
    .transact()
    .await?;
    assert!(outcome.is_success());

    let user_outcome_after_decrement = contract
        .view("get_count")
        .args_json(json!({}))
        .await?;
    assert_eq!(user_outcome_after_decrement.json::<u32>()?, 0);

  // **Test Reset:**
  // - Similar logic, calling "reset" and verifying count is reset to 0
  let outcome = user_account
    .call(contract.id(), "increment")
    .args_json(json!({}))
    .transact()
    .await?;
    assert!(outcome.is_success());

    let outcome = user_account
    .call(contract.id(), "reset")
    .args_json(json!({}))
    .transact()
    .await?;
    assert!(outcome.is_success());

    let user_outcome_after_decrement = contract
        .view("get_count")
        .args_json(json!({}))
        .await?;
    assert_eq!(user_outcome_after_decrement.json::<u32>()?, 0);

  Ok(())
}
