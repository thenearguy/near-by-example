## Contract: function_types
This contract shows different types of functions given by the Near SDK to create the smart contracts.

**This contract is deployed under `function-types.thenearguy.testnet`.**

### Contract Structure:

* **Dependencies:**
    * `near_sdk`: Provides core functionalities for building NEAR smart contracts.
        * `env`: Provides access to environmental variables like attached deposit and predecessor account ID.
        * `near`: Macro for defining functions callable from outside the contract.
        * `AccountId`: Represents an account ID on the NEAR blockchain.
        * `NearToken`: Represents the NEAR token used for transactions.
        * `Promise`: Represents an asynchronous operation for transferring NEAR tokens.

* **State Variable:**
    * `#[near(contract_state)]`: This attribute defines the structure `Contract` as the contract's state. Any changes to its fields will be persisted to the NEAR blockchain's storage.
    * `pub struct Contract { ... }`: Defines the `Contract` structure with the following fields:
        * `count: u32`: Stores the current counter value (unsigned 32-bit integer).
        * `deployer: AccountId`: Stores the account ID of the contract deployer.

* **Default Implementation:**
    * `impl Default for Contract { ... }`: This implementation provides a way to easily initialize a new `Contract` instance with default values:
        * `fn default() -> Self { ... }`: This function returns a new `Contract` with a counter value of 0 and the deployer's account ID retrieved using `env::predecessor_account_id()`.

### Contract Functions:

This contract manages a counter variable and provides functions to interact with it. Here's a breakdown considering function types:

* **Initialization:**
    * `init`: Private function that initializes the contract (called only once).

* **Counter Modification:**
    * `increment`: Public function to increase the counter by 1 (calls a helper function).
    * `decrement`: Public function to decrease the counter by 1.
    * `internal_increment`: Private helper function to increment by a specific value.
    * `boost`: Public payable function that increases the counter by 5 if attached deposit is greater than 1 NEAR.

* **Funds Management:**
    * `withdraw`: Public function for the deployer to withdraw funds from the contract.
    * `withdraw_private`: Private function for internal fund transfer (used by `withdraw`).

* **Read-Only Functions:**
    * `get_count`: Public function to retrieve the current counter value.
    * `deployer`: Public function to retrieve the deployer's account ID.
    * `balance`: Public function to retrieve the contract's NEAR token balance.
    * `return_one`: Public pure function that always returns 1 (for demonstration).
