# Contract: hello_near

This is a simple "Hello, World!" style smart contract for the NEAR blockchain written in Rust. It allows users to store and retrieve a greeting message. You can interact with the contract to:

* **get_greeting:** Retrieve the current greeting stored in the contract (defaults to "Hello").
* **set_greeting:** Change the greeting to a new message of your choice. 

**This contract is deployed under `hello-near.thenearguy.testnet`.**

## Deployment

#### How to Build Locally?

Install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
cargo near build
```
or
```bash
cargo build
```

#### How to Test Locally?

```bash
cargo test
```

#### How to Deploy?

Deployment is automated with GitHub Actions CI/CD pipeline.
To deploy manually, install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
cargo near deploy <yourid.testnet> without-init-call network-config testnet sign-with-keychain send
```

**Deployment Example**

1. Create a sub-account under your main account like `hello-near.thenearguy.testnet` under `thenearguy.testnet`

```bash
near create-account hello-near.thenearguy.testnet --masterAccount  thenearguy.testnet
```
output

```bash
Creating account hello-near.thenearguy.testnet.testnet using thenearguy.testnet
Transaction Id 5RgCfEYWJ8V2isdm9GGCE91ji7LMU5uaY7cvU2hgNu8K
Open the explorer for more info: https://testnet.nearblocks.io/txns/5RgCfEYWJ8V2isdm9GGCE91ji7LMU5uaY7cvU2hgNu8K
Storing credentials for account: hello-near.thenearguy.testnet (network: testnet)
Saving key to '~/.near-credentials/testnet/hello-near.thenearguy.testnet.json'
```

2. Send some near token to your newly created account
```bash
near send thenearguy.testnet hello-near.thenearguy.testnet 0.2
```
output
```bash
Sending 0.2 NEAR to hello-near.thenearguy.testnet from thenearguy.testnet
Transaction Id HJyYahUqT2xg8gTNgXPpqwp84TnPvHZAXuQ7qQgqAiTU
Open the explorer for more info: https://testnet.nearblocks.io/txns/HJyYahUqT2xg8gTNgXPpqwp84TnPvHZAXuQ7qQgqAiTU
```

3. Deploy
```bash
cargo near deploy hello-near.thenearguy.testnet without-init-call network-config testnet sign-with-keychain send
```
output
```bash
Unsigned transaction:

signer_id:    hello-near.thenearguy.testnet
receiver_id:  hello-near.thenearguy.testnet
actions: -- deploy contract

Your transaction was signed successfully.
Public key: ed25519:Apj5E1tCPVvMFBptvjaEA3Gp3K8K5JrWKyGhhhzbJBQS
Signature: ed25519:5keoU6GamGWip54ZqjFvxUL2dvy39jPLAg3d3UiMMTs24fUvLmZiAk7mbKmnCKUrGDzWM8Tzp7jnrxKECLXpWyy3
> How would you like to proceed? send      - Send the transaction to the network
Transaction sent ...
--- Logs ---------------------------
Logs [hello-near.thenearguy.testnet]:   No logs
--- Result -------------------------
Empty result
------------------------------------

Contract code has been successfully deployed.
Transaction ID: 8QC1GpA9dmXQCcwxYbaFNzeVL7HFFpG9t1PTRNNh2uPe
To see the transaction in the transaction explorer, please open this url in your browser:
https://explorer.testnet.near.org/transactions/8QC1GpA9dmXQCcwxYbaFNzeVL7HFFpG9t1PTRNNh2uPe
```

4. Calling the `get_greeting` function.
```bash
 near call hello-near.thenearguy.testnet get_greeting '{}' --accountId thenearguy.testnet
 ```
 output
 ```bash
 Scheduling a call: hello-near.thenearguy.testnet.get_greeting({})
Transaction Id 4hQXuKeGzpF7kpwscSXseX6AqdcmXgYP6EnQC338mjD1
Open the explorer for more info: https://testnet.nearblocks.io/txns/4hQXuKeGzpF7kpwscSXseX6AqdcmXgYP6EnQC338mjD1
'Hello'
```

5. Setting a new greeting message with `set_greeting` function
```bash
near call hello-near.thenearguy.testnet set_greeting '{"greeting": "Hello, Near!"}' --accountId thenearguy.testnet
```
output
```bash
Scheduling a call: hello-near.thenearguy.testnet.set_greeting({"greeting": "Hello, Near!"})
Receipt: GYjF8tkmbyfwH7MFKCmmcUNqMj96vaTwYUMgN9HGrh6R
        Log [hello-near.thenearguy.testnet]: Saving greeting: {greeting}
Transaction Id 7CzQQDBBXHmquQjtiUZkaRmNk32xD1s5jdv1oC5mejiG
Open the explorer for more info: https://testnet.nearblocks.io/txns/7CzQQDBBXHmquQjtiUZkaRmNk32xD1s5jdv1oC5mejiG
''
```

6. Getting the new greeting message with `set_greeting` function
```bash
near call hello-near.thenearguy.testnet get_greeting '{}' --accountId thenearguy.testnet
```
output
```bash
Scheduling a call: hello-near.thenearguy.testnet.get_greeting({})
Transaction Id FBJkjyqupfSnZBo6HebEHaUuGXktKm5NSj5rw6283M5
Open the explorer for more info: https://testnet.nearblocks.io/txns/FBJkjyqupfSnZBo6HebEHaUuGXktKm5NSj5rw6283M5
'Hello, Near!'
```
## Contract Structure

1. **Imports:**
    - `near_sdk`: Provides core functionalities for building NEAR smart contracts.
    - `log`: Used for logging messages to the NEAR blockchain.
    - `near`: Used for accessing NEAR-specific functions (not used in this example).

2. **Contract Structure:**
    - `Contract`: This struct defines the state of the contract, which stores a single string named `greeting`.
    - `#[near(contract_state)]`: This attribute tells NEAR that `Contract` holds the contract's state.

3. **Default Implementation:**
    - `impl Default for Contract`: This allows automatic initialization of the contract with a default greeting.
    - `Self { greeting: "Hello".to_string() }`: Sets the default greeting to "Hello".

4. **Contract Methods:**
    - `#[near]`: This attribute indicates that the following methods can be called from outside the contract.
    - `pub fn get_greeting(&self) -> String`: This public method returns the current greeting stored in the contract.
    - `pub fn set_greeting(&mut self, greeting: String)`: This public method allows changing the greeting by accepting a new string argument. It logs the new greeting before updating the internal state.

5. **Inline Tests:**
    - `#[cfg(test)]`: This section defines unit tests that are only run during development, not when deployed on the NEAR blockchain.
    - Two tests are included:
        - `get_default_greeting`: Checks if the default greeting is returned correctly.
        - `set_then_get_greeting`: Tests if setting a new greeting and then retrieving it works as expected.

**Overall, this simple contract showcases the foundation for building more complex NEAR smart contracts in Rust.**

## Useful Links

- [cargo-near](https://github.com/near/cargo-near) - NEAR smart contract development toolkit for Rust
- [near CLI](https://near.cli.rs) - Iteract with NEAR blockchain from command line
- [NEAR Rust SDK Documentation](https://docs.near.org/sdk/rust/introduction)
- [NEAR Documentation](https://docs.near.org)
- [NEAR StackOverflow](https://stackoverflow.com/questions/tagged/nearprotocol)
- [NEAR Discord](https://near.chat)
- [NEAR Telegram Developers Community Group](https://t.me/neardev)
- NEAR DevHub: [Telegram](https://t.me/neardevhub), [Twitter](https://twitter.com/neardevhub)
