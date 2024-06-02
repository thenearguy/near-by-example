# Contract: vectors

## NEAR Smart Contract for Vector Manipulation

This is a NEAR smart contract that provides functionalities for managing a vector of unsigned 32-bit integers (u32).

**This contract is deployed under `vectors.thenearguy.testnet`**

### Features

* **Vector Storage:** Stores and manages a vector of u32 elements.
* **Element Addition:** Allows adding elements to the end of the vector using `push_elements`.
* **Length Check:** Provides the ability to get the current number of elements in the vector using `length`.
* **Element Replacement:** Enables replacing an element at a specific index with a new value using `replace_elements`.
* **Vector Extension:** Supports extending the vector with additional elements from a fixed-size array using `extend_vector`.
* **Element Removal:** Offers functionalities to remove elements:
    * `pop_elements`: Removes the last element from the vector.
    * `remove_elements`: Removes an element at a specific index.
* **Empty Check:** Determines if the vector is empty using `check_empty`.
* **Vector Clearing:** Allows clearing all elements from the vector using `clear_vector`.
* **Vector Iterating:** Allows iterating over a vector using `iter()`.
* **Vector Iterating and changing the value:** Allows iterating over the vector and changing the value using `iter_mut()`.

### Usage

This contract can be used for various scenarios where you need to store and manage a list of values in a NEAR smart contract. For example, you could use it to:

* Maintain a list of user IDs.
* Track high scores in a game.
* Store voting results.
