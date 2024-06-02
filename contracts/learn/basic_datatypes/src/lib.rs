// Import core NEAR functionalities
use near_sdk::near;

// Import JSON serialization types from the NEAR SDK
use near_sdk::json_types::{U64, U128, I64, I128};

#[near(contract_state)]
pub struct Contract {
    // String data
    string: String,

    // Signed integer data types
    num_i8: i8,
    num_i16: i16,
    num_i32: i32,

    // Unsigned integer data types
    num_u8: u8,
    num_u16: u16,
    num_u32: u32,

    // Boolean data
        is_bool: bool,
/*
    Why are i64, i128, u64, u128 not supported directly?

    - Storage Efficiency: NEAR contracts store data on the blockchain, and efficiency is crucial. While u64 and u128 can represent larger numbers, they might require more storage space compared to u32. NEAR optimizes for smaller data types for common use cases.

    - Serialization: When interacting with the contract (e.g., calling methods), data needs to be serialized (converted) to a format suitable for transmission. The json_types module handles this conversion for U64, U128, I64, and I128, ensuring compatibility with NEAR's communication protocols.
*/

    // Larger signed and unsigned integer data types (not directly supported by NEAR)
    num_i64: I64,
    num_i128: I128,
    num_u64: U64,
    num_u128: U128,
}

impl Default for Contract {
    // Default values for contract state fields
    fn default() -> Self {
        Self {
        string: "Hello".to_string(),
        num_i8: -1,
        num_i16: 22,
        num_i32: 333,
        num_u8: 4,
        num_u16: 55,
        num_u32: 666,
        is_bool: true,
        num_i64: I64(89899898),
        num_i128: I128(-123423452123),
        num_u64: U64(12797234123),
        num_u128: U128(12345567890128988),
        }
    }
}

#[near]
impl Contract {
    // Getter methods for each data field (returning appropriate types)
    pub fn string(&self) -> String {
        self.string.clone()
    }

    pub fn num_i8(&self) -> i8 {
        self.num_i8
    }

    pub fn num_i16(&self) -> i16 {
        self.num_i16
    }

    pub fn num_i32(&self) -> i32 {
        self.num_i32
    }

    pub fn num_u8(&self) -> u8 {
        self.num_u8
    }

    pub fn num_u16(&self) -> u16 {
        self.num_u16
    }

    pub fn num_u32(&self) -> u32 {
        self.num_u32
    }

    pub fn is_bool(&self) -> bool {
        self.is_bool
    }

    pub fn num_i64(&self) -> I64 {
        self.num_i64
    }

    pub fn num_i128(&self) -> I128 {
        self.num_i128
    }

    pub fn num_u64(&self) -> U64 {
        self.num_u64
    }

    pub fn num_u128(&self) -> U128 {
        self.num_u128
    }
}
