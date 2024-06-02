# Contract: counter

This is a basic NEAR smart contract that implements a counter with functionality to increment, decrement, and reset the count. It's designed for educational purposes and demonstrates fundamental concepts of NEAR smart contract development.

**This contract is deployed under `counter.thenearguy.testnet`.**

**Features:**

* Stores a counter value (unsigned 32-bit integer).
* Provides methods for:
    * `get_count`: Retrieves the current counter value.
    * `increment`: Increases the counter value by 1.
    * `decrement`: Decreases the counter value by 1.
    * `reset`: Resets the counter value to 0.

**Contract Code Breakdown:**

The contract code is located in the `contract.rs` file. Here's a brief explanation of each section:

**1. Imports:**

- `use near_sdk::near;`: Imports the `near` module from the `near_sdk` crate, providing access to NEAR-specific functionalities.

**2. Contract Structure:**

- `#[near(contract_state)]`: This attribute defines the `Contract` struct as the contract's state, which holds the data it manages.
- `pub struct Contract {`: Defines the `Contract` struct with a single field:
    - `count: u32`: This field stores the current counter value as an unsigned 32-bit integer.

**3. Default Implementation:**

- `impl Default for Contract { ... }`: This section implements the `Default` trait for the `Contract` struct. It defines a default constructor that initializes a new `Contract` with a count of 0.

**4. Contract Methods:**

- `#[near]`: This attribute indicates the following methods are accessible for external calls.
- `pub fn get_count(&self) -> u32 { ... }`: This public method named `get_count` retrieves the current value of the `count` field and returns it as a u32 value. It uses `&self` to indicate it's a view method (doesn't modify state).
- `pub fn increment(&mut self) { ... }`: This public method named `increment` increases the `count` value by 1. It uses `&mut self` to indicate it modifies the contract's state.
- Similar logic applies to `decrement` and `reset` methods.

**5. Unit Tests (tests directory):**

- The `tests` directory contains unit tests written in Rust using the `near_workspaces` library. These tests simulate interactions with the contract and verify its functionality.

