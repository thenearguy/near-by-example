use near_sdk::{env, near, AccountId, NearToken, Promise};

// This #[near(contract_state)] attribute defines the structure 'Contract'
// as the contract's state. Any changes to its fields will be persisted to storage.
#[near(contract_state)]
pub struct Contract {
    // A counter variable named 'count' to store a value (u32).
    count: u32,
    // Stores the deployer's account ID (AccountId).
    deployer: AccountId,
}

impl Default for Contract {
    // This default implementation provides a way to easily initialize a new 'Contract'
    // with a count of 0 and the deployer's account ID.
    fn default() -> Self {
        Self {
            count: 0,
            deployer: env::predecessor_account_id(),
        }
    }
}

#[near]
impl Contract {
    // An initialization function with #[init] and #[private] attributes.
    // This function (if present) must be called first (private) to initialize the contract.
    #[init]
    #[private]
    pub fn init() -> Self {
        Self {
            count: 0,
            deployer: env::predecessor_account_id(),
        }
    }

    // Function to increment the counter by 1 (calls the internal_increment helper function).
    pub fn increment(&mut self) {
        self.internal_increment(1);
    }

    // Internal helper function to increment the counter by a specific value.
    // Not directly exposed to the outside world (private function).
    fn internal_increment(&mut self, value: u32) {
        self.count += value;
    }

    // Function to decrement the counter by 1 (state-changing function).
    pub fn decrement(&mut self) {
        self.count -= 1;
    }

    // Function to boost the counter by 5 if attached deposit is greater than 1 NEAR (payable function).
    #[payable]
    pub fn boost(&mut self) {
        assert!(env::attached_deposit() > NearToken::from_millinear(1));
        self.internal_increment(5);
    }

    // Function to withdraw funds from the contract (only deployer can call).
    pub fn withdraw(&mut self, amount: u128) -> Promise {
        assert!(self.deployer == env::predecessor_account_id());
        self.withdraw_private(env::predecessor_account_id(), amount)
    }

    // Private function to transfer funds (used internally by the withdraw function).
    #[private]
    pub fn withdraw_private(&mut self, recipient: AccountId, amount: u128) -> Promise {
        Promise::new(recipient).transfer(NearToken::from_yoctonear(amount))
    }

    // Function to get the current counter value (read-only function).
    pub fn get_count(&self) -> u32 {
        self.count
    }

    // Function to get the deployer's account ID (read-only function).
    pub fn deployer(&self) -> AccountId {
        self.deployer.clone()
    }

    // Function to get the contract's NEAR token balance (read-only function).
    pub fn balance(&self) -> NearToken {
        env::account_balance()
    }

    // Pure function that always returns the value 1 (doesn't access state).
    pub fn return_one() -> u32 {
        1
    }
}
