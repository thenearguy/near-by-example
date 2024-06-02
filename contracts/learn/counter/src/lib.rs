// Import the `near` module from the `near_sdk` crate
use near_sdk::near;

// Define the contract structure named `Contract` with a state field `count`
#[near(contract_state)]
pub struct Contract {
    // Stores the current count value (unsigned 32-bit integer)
    count: u32,
}

// Default implementation for the `Contract` struct
impl Default for Contract {
    fn default() -> Self {
        // Initializes a new `Contract` with a default count of 0
        Self {
            count: 0,
        }
    }
}

// Implementations of the contract methods
#[near]
impl Contract {
    // Public method to get the current count value
    pub fn get_count(&self) -> u32 {
        // Return a copy of the count to avoid modifying the internal state
        self.count
    }

    // Public method to increment the count by 1
    pub fn increment(&mut self) {
        // Increase the count value by 1
        self.count += 1;
    }

    // Public method to decrement the count by 1
    pub fn decrement(&mut self) {
        // Decrease the count value by 1
        self.count -= 1;
    }

    // Public method to reset the count to 0
    pub fn reset(&mut self) {
        // Set the count value to 0
        self.count = 0;
    }
}

// Unit tests for the contract functionality
#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a default contract instance
    fn default() -> Contract {
        Contract::default()
    }

    // Test case for the default constructor (checks initial count)
    #[test]
    fn test_default() {
        let contract: Contract = default();
        assert_eq!(contract.count, 0);
    }

    // Test case for the `increment` method (checks incrementing count)
    #[test]
    fn test_increment() {
        let mut contract: Contract = default();
        assert_eq!(contract.count, 0);

        contract.increment();
        assert_eq!(contract.count, 1);
    }

    // Test case for the `decrement` method (checks decrementing count)
    #[test]
    fn test_decrement() {
        let mut contract: Contract = default();
        assert_eq!(contract.count, 0);

        contract.increment();
        assert_eq!(contract.count, 1);

        contract.decrement();
        assert_eq!(contract.count, 0);
    }

    // Test case for the `reset` method (checks resetting count)
    #[test]
    fn reset() {
        let mut contract: Contract = default();
        assert_eq!(contract.count, 0);

        contract.increment();
        assert_eq!(contract.count, 1);

        contract.reset();
        assert_eq!(contract.count, 0);
    }
}