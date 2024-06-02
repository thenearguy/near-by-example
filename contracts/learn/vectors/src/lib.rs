// Import necessary functionality from the NEAR SDK
use near_sdk::{near, store::Vector};

// Define the `Contract` struct to store the state of our contract
#[near(contract_state)]
pub struct Contract {
  // Create a vector named `vector` to store elements of type `u32` (unsigned 32-bit integers)
  vector: Vector<u32>,
}

// Implement the `Default` trait for the `Contract` struct
impl Default for Contract {
    // This function defines the default state of the contract when it's first created
    // **Explanation:**
    //   - `fn default() -> Self` specifies that this function defines the default behavior for the `Default` trait.
    //   - It returns a `Self` which refers to the `Contract` struct itself.
    //   - Inside the function body, we create a new `Contract` with an empty vector (`Vector::new(b"vec-uid-1".to_vec())`) but this initialization can likely be removed (see explanation below).

    // The initial line in the Default implementation (vector: Vector::new(b"vec-uid-1".to_vec())) likely served a
    // placeholder purpose during development.
    // In NEAR storage, prefixes are typically used to avoid collisions with other
    // collections. However, for this specific case, creating an empty vector by removing the initialization line
    // altogether is more suitable.
    fn default() -> Self {
        Self {
        vector: Vector::new(b"vec-uid-1".to_vec()),
        }
    }
}

// Implement methods on the `Contract` struct using the `#[near]` attribute
#[near]
impl Contract {

    // Function to add elements to the end of the vector
    // **Explanation:**
    //   - `pub fn push_elements(&mut self)`: This function is public (`pub`) and allows modification (`&mut self`) of the contract state.
    //   - It iterates five times, pushing values 1 to 5 onto the vector using `self.vector.push(value)`.
    pub fn push_elements(&mut self) {
        self.vector.push(1);
        self.vector.push(2);
        self.vector.push(3);
        self.vector.push(4);
        self.vector.push(5);
    }

    // Function to get the length of the vector (number of elements)
    // **Explanation:**
    //   - `pub fn length(&self) -> u32`: This function is public (`pub`) and doesn't modify (`&self`) the contract state.
    //   - It returns the length of the vector using `self.vector.len()` and converts it to a `u32` type.
    pub fn length(&self) -> u32 {
        self.vector.len()
    }

    // Function to replace an element at a specific index with a new value
    // **Explanation:**
    //   - `pub fn replace_elements(&mut self, index: u32, value: u32)`: This function allows modification (`&mut self`).
    //   - It takes two arguments: `index` (the position of the element to replace) and `value` (the new value).
    //   - It uses `self.vector.replace(index, value)` to replace the element at the specified index with the provided value.
    pub fn replace_elements(&mut self, index: u32, value: u32) {
        self.vector.replace(index, value);
    }

    // Function to extend the vector with additional elements from a fixed-size array
    // **Explanation:**
    //   - `pub fn extend_vector(&mut self, adding: [u32; 2])`: This function allows modification (`&mut self`).
    //   - It takes a fixed-size array `adding` of type `[u32; 2]` containing two `u32` elements.
    //   - It uses `self.vector.extend(adding)` to efficiently append all elements from the array to the end of the vector.
    pub fn extend_vector(&mut self, adding: [u32; 2]) {
        self.vector.extend(adding);
    }

    // Function to remove the last element from the vector
    // **Explanation:**
    //   - `pub fn pop_elements(&mut self)`: This function allows modification (`&mut self`).
    //   - It uses `self.vector.pop()` to remove and return the last element from the vector.
    pub fn pop_elements(&mut self) {
        self.vector.pop();
    }

    // Function to remove an element at a specific index
    // **Explanation:**
    //   - `pub fn remove_elements(&mut self, index: u32)`: This function allows modification (`&mut self`).
    //   - It takes an argument `index` (the position of the element to remove) of type `u32`.
    //   - It uses `self.vector.swap_remove(index)` to remove the element at the specified index. This method efficiently removes the element by swapping it with the last element and then popping the last element (which is now the element we wanted to remove).
    pub fn remove_elements(&mut self, index: u32) {
        self.vector.swap_remove(index);
    }
    
    // Function to check if the vector is empty
    // **Explanation:**
    //   - `pub fn check_empty(&self) -> bool`: This function doesn't modify (`&self`) the contract state and returns a boolean (`bool`).
    //   - It uses `self.vector.is_empty()` to check if the vector has any elements and returns `true` if empty, `false` otherwise.
    pub fn check_empty(&self) -> bool {
        self.vector.is_empty()
    }
    
    // Function to clear all elements from the vector
    // **Explanation:**
    //   - `pub fn clear_vector(&mut self)`: This function allows modification (`&mut self`).
    //   - It uses `self.vector.clear()` to remove all elements from the vector, effectively making it empty.
    pub fn clear_vector(&mut self) {
        self.vector.clear();
    }

    // Iterate over vector using iter()
    pub fn iterate(&self) {
        for _element in self.vector.iter() {
            // TODO
        }
    }

    // You can iterate over vector and change their values with iter_mut()
    pub fn iterate_and_change(&mut self) {
        for element in self.vector.iter_mut() {
            *element += 1;
        }
    }
}
