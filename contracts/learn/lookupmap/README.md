## Contract: lookupmap

This is a NEAR smart contract written in Rust that demonstrates how to use a `LookupMap` for storing key-value pairs. The contract allows you to interact with data using String keys and u32 values.

**This contract is deployed under `lookupmap.thenearguy.testnet`.**

**Functionality:**

The contract provides functions for various CRUD (Create, Read, Update, Delete) operations on key-value pairs:

**Insertion:**

* `insert_values(&mut self)`: Inserts a new key-value pair into the `hashmap`.
* `insert_and_returns(&mut self) -> Option<u32>`: Inserts a key-value pair and returns the previous value (if any) associated with the key.
* `insert_with_set(&mut self)`: Inserts a key-value pair using `set` with an `Option` value, explicitly handling the possibility of no previous value. 

**Retrieval:**

* `get_values(&self) -> Option<&u32>`: Retrieves the value associated with a key, returning `Some(&value)` if it exists, otherwise `None`.
* `get_values_with_index(&self) -> u32` ( **Less Safe** ): Retrieves the value using indexing notation, but this can cause a panic if the key doesn't exist. It's generally recommended to use `get_values` for safer retrieval.

**Modification:**

* `get_and_change_values(&mut self)`: Retrieves a value by its key and modifies it in-place (if it exists).

**Existence Check:**

* `is_exist(&self) -> bool`: Checks if a specific key exists in the `hashmap`, returning `true` if it exists and `false` otherwise.

**Deletion:**

* `remove_key(&mut self) -> Option<u32>`: Removes a key-value pair from the `hashmap`, returning the previous value (if any).
* `remove_with_set(&mut self)`: Removes a key-value pair by setting its value to `None`.

**Initialization with Default Value:**

* `check_if_not_initialize(&mut self) -> &mut u32`: Checks if a key exists and initializes it with a default value (3 in this case) if not. It returns a mutable reference to the value.
