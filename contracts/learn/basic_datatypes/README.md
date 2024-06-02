## Contract: basic_datatypes

This is a basic NEAR smart contract demonstrating how to store and retrieve different data types, including strings, integers (various sizes), and booleans. It highlights the use of helper types for larger integer data types not directly supported by NEAR.

**This contract is deployed under `basic-datatypes.thenearguy.testnet`.**

**Features:**

* Stores string data.
* Stores signed and unsigned integers of various sizes:
    * `i8`, `i16`, `i32` (directly supported by NEAR)
    * `i64`, `i128` (supported using `I64`, `I128` helper types)
    * `u8`, `u16`, `u32` (directly supported by NEAR)
    * `u64`, `u128` (supported using `U64`, `U128` helper types)
* Stores boolean values.

**Code Breakdown:**

* The contract state (`Contract`) uses native types for strings (`String`) and booleans (`bool`).
* For larger integers (`i64`, `i128`, `u64`, and `u128`), the contract uses helper types (`I64`, `I128`, `U64`, and `U128`) from the `near_sdk::json_types` module.

**Explanation:**

NEAR optimizes for smaller data types for common use cases. The helper types (`I64`, `I128`, `U64`, and `U128`) ensure compatibility with NEAR's communication protocols for larger integers during serialization (conversion for data transmission).

**Why i64, i128, u64, u128 not supported directly?**

**Storage Efficiency:** NEAR contracts store data on the blockchain, and efficiency is crucial. While u64 and u128 can represent larger numbers, they might require more storage space compared to u32. NEAR optimizes for smaller data types for common use cases.

**Serialization:** When interacting with the contract (e.g., calling methods), data needs to be serialized (converted) to a format suitable for transmission. The json_types module handles this conversion for U64, U128, I64, and I128, ensuring compatibility with NEAR's communication protocols.
