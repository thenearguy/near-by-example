use near_sdk::{env, near, AccountId, BlockHeight, Gas, NearToken, StorageUsage};

#[near(contract_state)]
pub struct Contract {}

impl Default for Contract {
  // This default implementation provides a way to easily initialize a new 'Contract'
  // with an empty state.
  fn default() -> Self {
    Self {}
  }
}

#[near]
impl Contract {
  // Returns the account ID of the account that called the current function.
  pub fn get_function_caller(&self) -> AccountId {
    // Access the environment variable to get the predecessor account ID (caller)
    env::predecessor_account_id()
  }

  // Returns the account ID of the contract itself.
  pub fn get_contract_address(&self) -> AccountId {
    // Access the environment variable to get the current contract's account ID
    env::current_account_id()
  }

  // Returns the account ID of the account that signed the transaction calling the function.
  pub fn get_transaction_signer(&self) -> AccountId {
    // Access the environment variable to get the signer account ID
    env::signer_account_id()
  }

  // Returns the amount of NEAR tokens attached to the function call.
  pub fn get_call_balance(&self) -> NearToken {
    // Access the environment variable to get the attached deposit
    env::attached_deposit()
  }

  // Returns the balance of NEAR tokens held by the contract.
  pub fn get_contract_balance(&self) -> NearToken {
    // Access the environment variable to get the contract's account balance
    env::account_balance()
  }

  // Returns the amount of gas available for the execution of the current function.
  pub fn get_gas_available(&self) -> Gas {
    // Access the environment variable to get the prepaid gas
    env::prepaid_gas()
  }

  // Returns the current timestamp of the block when the function is called.
  pub fn get_current_timestamp(&self) -> u64 {
    // Access the environment variable to get the current block timestamp
    env::block_timestamp()
  }

  // Returns the current height of the blockchain.
  pub fn get_block_height(&self) -> BlockHeight {
    // Access the environment variable to get the current block height
    env::block_height()
  }

  // Returns the amount of storage currently used by the contract.
  pub fn get_storage_used(&self) -> StorageUsage {
    // Access the environment variable to get the storage usage
    env::storage_usage()
  }

  // Returns the current storage cost per byte in yoctonear (smallest NEAR denomination).
  pub fn get_cost_per_byte(&self) -> NearToken {
    // Access the environment variable to get the storage byte cost
    env::storage_byte_cost()
  }

  // Returns the amount of gas used by the current function execution.
  pub fn get_gas_used(&self) -> Gas {
    // Access the environment variable to get the used gas
    env::used_gas()
  }

  // Returns the amount of NEAR tokens currently locked in the contract (e.g., by other contracts).
  pub fn get_contract_locked_balance(&self) -> NearToken {
    // Access the environment variable to get the account locked balance
    env::account_locked_balance()
  }
}
