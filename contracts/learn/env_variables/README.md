# Contracts: env_variables

This code defines a simple NEAR smart contract that exposes various functions to retrieve information about the execution environment. Here's a breakdown of the code with comments:

**This contract is deployed under `env-variables.thenearguy.testnet`.**

**Contract Definition:**

* `#[near_sdk::contract_state]`: This attribute declares the `Contract` struct as the contract state. It stores the data associated with the contract.
* `impl Default for Contract`: This defines a default constructor for the `Contract` struct. It creates an empty instance with no initial state.

**Function Implementations:**

The contract implements several functions marked with the `#[near]` attribute. These functions are accessible by external accounts to interact with the contract. Each function has comments explaining its purpose and the information it retrieves:

* `get_function_caller`: Returns the account ID of the account that called the current function.
* `get_contract_address`: Returns the account ID of the contract itself (the current contract).
* `get_transaction_signer`: Returns the account ID of the account that signed the transaction calling the function.
* `get_call_balance`: Returns the amount of NEAR tokens attached to the function call.
* `get_contract_balance`: Returns the current balance of NEAR tokens held by the contract.
* `get_gas_available`: Returns the amount of gas available for the execution of the current function.
* `get_current_timestamp`: Returns the current timestamp of the block when the function is called.
* `get_block_height`: Returns the current height of the blockchain.
* `get_storage_used`: Returns the amount of storage currently used by the contract.
* `get_cost_per_byte`: Returns the current storage cost per byte in yoctonear (smallest NEAR denomination).
* `get_gas_used`: Returns the amount of gas used by the current function execution.
* `get_contract_locked_balance`: Returns the amount of NEAR tokens currently locked in the contract (e.g., by other contracts).

**Overall, this contract serves as a basic example to demonstrate how to access information about the NEAR execution environment from within a smart contract.** 