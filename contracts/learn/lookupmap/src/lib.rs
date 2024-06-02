// Import necessary functionality from the NEAR SDK
use near_sdk::{near, store::LookupMap};

// Define the `Contract` struct to store the state of our contract
#[near(contract_state)]
pub struct Contract {
    // Create a LookupMap named `hashmap` to store key-value pairs (String keys and u32 values)
    hashmap: LookupMap<String, u32>,
}

// Implement the `Default` trait for the `Contract` struct
impl Default for Contract {
    // This function defines the default state of the contract when it's first created
    fn default() -> Self {
        Self {
        // Initialize an empty LookupMap with a unique prefix ("map-uid-1") to avoid collisions with other collections
        hashmap: LookupMap::new(b"map-uid-1".to_vec()),
        }
    }
}

// Implement methods on the `Contract` struct using the `#[near]` attribute
#[near]
impl Contract {
    // Function to insert a key-value pair into the LookupMap
    // **Explanation:**
    //   - `pub fn insert_values(&mut self)`: This function allows modification (`&mut self`) of the contract state.
    //   - It inserts the key "bob" (converted to a string) and the value 1 into the LookupMap using `self.hashmap.insert()`.
    pub fn insert_values(&mut self) {
        self.hashmap.insert("bob".to_string(), 1);
    }

    // Function to insert a key-value pair and return the previous value (if any)
    // **Explanation:**
    //   - `pub fn insert_and_returns(&mut self) -> Option<u32>`: This function allows modification (`&mut self`) and returns an `Option<u32>`.
    //   - It inserts the key "alice" and the value 2. If a previous value existed for "alice", it's returned as `Some(value)`. Otherwise, `None` is returned.
    pub fn insert_and_returns(&mut self) -> Option<u32> {
        self.hashmap.insert("alice".to_string(), 2)
    }

    // Function to insert a key-value pair using `set` with an `Option` value
    // **Explanation:**
    //   - `pub fn insert_with_set(&mut self)`: This function allows modification (`&mut self`).
    //   - It uses `self.hashmap.set("john".to_string(), Some(3))` to insert the key "john" and the value 3 wrapped in `Some`. This approach explicitly handles the possibility of no previous value.
    pub fn insert_with_set(&mut self) {
        self.hashmap.set("john".to_string(), Some(3));
    }

    // Function to retrieve a value by its key from the LookupMap
    // **Explanation:**
    //   - `pub fn get_values(&self) -> Option<&u32>`: This function doesn't modify (`&self`) the contract state and returns an `Option<&u32>`.
    //   - It uses `self.hashmap.get("bob")` to get the value associated with the key "bob". It returns `Some(&value)` if the key exists, otherwise `None`.
    pub fn get_values(&self) -> Option<&u32> {
        self.hashmap.get("bob")
    }

    // Function to retrieve a value by its key and modify it in-place
    // **Explanation:**
    //   - `pub fn get_and_change_values(&mut self)`: This function allows modification (`&mut self`).
    //   - It uses `self.hashmap.get_mut("bob")` to get a mutable reference (`&mut u32`) to the value associated with "bob" (if it exists).
    //   - If the key exists (`Some(x)`), the value is dereferenced (`*x`) and updated to 10.
    pub fn get_and_change_values(&mut self) {
        if let Some(x) = self.hashmap.get_mut("bob") {
        *x = 10;
        }
    }

      // Function to retrieve a value by its key using indexing notation (potentially panics)
    // **Explanation:**
    //   - `pub fn get_values_with_index(&self) -> u32`: This function doesn't modify (`&self`) the contract state and returns a `u32`.
    //   - It retrieves the value associated with the key "bob" using indexing notation `self.hashmap["bob"]`.
    //   - This approach is concise but less safe. If the key doesn't exist, it will cause a panic by calling `unwrap()` on a potential `None` value. It's generally recommended to use `get` and handle the `Option` result explicitly.
    pub fn get_values_with_index(&self) -> u32 {
        self.hashmap["bob"]
    }

    // Function to check if a key exists in the LookupMap
    // **Explanation:**
    //   - `pub fn is_exist(&self) -> bool`: This function doesn't modify (`&self`) the contract state and returns a `bool`.
    //   - It uses `self.hashmap.contains_key("bob")` to check if the key "bob" exists in the LookupMap. It returns `true` if the key exists, otherwise `false`.
    pub fn is_exist(&self) -> bool {
        self.hashmap.contains_key("bob")
    }

    // Function to remove a key-value pair from the LookupMap
    // **Explanation:**
    //   - `pub fn remove_key(&mut self) -> Option<u32>`: This function allows modification (`&mut self`) and returns an `Option<u32>`.
    //   - It removes the key "bob" from the LookupMap using `self.hashmap.remove("bob")`. If the key existed, the previous value is returned as `Some(value)`. Otherwise, `None` is returned.
    pub fn remove_key(&mut self) -> Option<u32> {
        self.hashmap.remove("bob")
    }

    // Function to remove a key-value pair by setting its value to None
    // **Explanation:**
    //   - `pub fn remove_with_set(&mut self)`: This function allows modification (`&mut self`).
    //   - It removes the key "john" by setting its value to `None` using `self.hashmap.set("john".to_string(), None)`. This effectively removes the key-value pair.
    pub fn remove_with_set(&mut self) {
        self.hashmap.set("john".to_string(), None);
    }

    // Function to check if a key exists and initialize it with a value if not
    // **Explanation:**
    //   - `pub fn check_if_not_initialize(&mut self) -> &mut u32`: This function allows modification (`&mut self`) and returns a mutable reference (`&mut u32`).
    //   - It uses `self.hashmap.entry("john".to_string()).or_insert(3)`. This is a concise way to check if the key "john" exists.
    //     - If the key exists, it returns a mutable reference to the existing value.
    //     - If the key doesn't exist, it inserts the key "john" with the value 3 and returns a mutable reference to the newly inserted value.
    pub fn check_if_not_initialize(&mut self) -> &mut u32 {
        self.hashmap.entry("john".to_string()).or_insert(3)
    }
}
