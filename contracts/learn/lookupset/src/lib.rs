use near_sdk::{near, store::LookupSet};

// This #[near(contract_state)] attribute defines the structure 'Contract'
// as the contract's state. Any changes to its fields will be persisted to storage.
#[near(contract_state)]
pub struct Contract {
    // A LookupSet named 'set' is used to store elements of type u32 (unsigned 32-bit integers).
    // LookupSet offers efficient membership checks and avoids duplicate entries.
    set: LookupSet<u32>,
}

// This `Default` implementation provides a way to easily initialize a new 'Contract'
// with an empty LookupSet.
impl Default for Contract {
    fn default() -> Self {
        Self {
            set: LookupSet::new(b"set_uid_1".to_vec()),
        }
    }
}

#[near]
impl Contract {
    // This function `insert_elements` adds elements (u32 values) to the 'set' LookupSet.
    // Since LookupSet avoids duplicates, inserting the same value again will be ignored.
    pub fn insert_elements(&mut self) {
        self.set.insert(1);
        self.set.insert(2);
        self.set.insert(3);
        self.set.insert(4);
        self.set.insert(5);
    }

    // This function `is_present` checks if a specific value (u32) exists within the 'set' LookupSet.
    // It returns true if the value is present, false otherwise.
    pub fn is_present(&self, value: u32) -> bool {
        self.set.contains(&value)
    }

    // This function `remove_elements` removes a specific value (u32) from the 'set' LookupSet.
    // If the value exists, it will be deleted. This function does not return the deleted value.
    pub fn remove_elements(&mut self, value: u32) {
        self.set.remove(&value);
    }
}

// This #[cfg(test)] block defines unit tests for the contract functions.
// These tests are only compiled and run when the 'test' feature is enabled.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        // Create a new 'Contract' instance with default values.
        let mut contract: Contract = Contract::default();

        // Call the `insert_elements` function to add elements to the set.
        contract.insert_elements();

        // Assert that the value 5 is present in the set using the `is_present` function.
        assert_eq!(contract.set.contains(&5), true);
    }

    #[test]
    fn test_is_present() {
        let mut contract: Contract = Contract::default();
        contract.insert_elements();

        // Assert that the value 4 is present in the set using the `is_present` function.
        assert!(contract.is_present(4));
    }

    #[test]
    fn test_remove() {
        let mut contract: Contract = Contract::default();
        contract.insert_elements();

        // Assert that the value 4 is present before removal.
        assert!(contract.is_present(4));

        // Call `remove_elements` to remove the value 4.
        contract.remove_elements(4);

        // Assert that the value 4 is no longer present after removal.
        assert!(!contract.is_present(4));
    }
}
