**Title:**

## Hands-on Near Smart Contract Examples in Rust (NEAR-by-Example)

**Description:**

This repository provides a collection of smart contract examples written in Rust to help you learn NEAR blockchain smart contract development. It's designed with beginners in mind, offering a structured approach with increasing difficulty levels.

#### **All contracts are deployed under `thenearguy.testnet`.**

**Key Features:**

- **Clear Learning Path:** Progress from basic to advanced smart contracts, building upon foundational knowledge.
- **Full Rust Packages:** Each contract example is a complete Rust package, easy to set up and understand.
- **Optional Unit Tests:** Many examples include unit tests showcasing good testing practices for smart contracts.
- **Valuable Resources:** The `resources` folder offers links to tutorials, documentation, and other materials for learning Rust and NEAR concepts.

**Getting Started:**

Before diving in, ensure you have Rust and the NEAR tooling (near-cli-rs, cargo-near) installed. Refer to the official NEAR documentation for setup instructions: [https://docs.near.org/build/smart-contracts/quickstart](https://docs.near.org/build/smart-contracts/quickstart)

**Folder Structure:**

```
near-by-example/
│ 
├── resources/
│   ├── rust_resources.md    # Links for learning Rust
│   └── near_resources.md    # Links for understanding NEAR blockchain & smart contracts
│ 
├── contracts/
│   ├── learn/               # Easy level contract examples
│   │   ├── hello_near/     
│   │   │   ├── Cargo.toml
│   │   │   ├── src/
│   │   │   │   └── lib.rs   # Contract code
│   │   │   └── tests/       # Optional, unit tests
│   │   │    
│   ├── practice/            # Intermediate level contracts examples
│   │   │ 
│   └── master/              # Advance level contract examples
└────────────────────
```

**Structure:**

* **Main Folders:**
    * **resources/:** Stores external learning resources.
        * **rust_resources.md:** Links for learning Rust relevant to smart contract development.
        * **near_resources.md:** Links for understanding NEAR blockchain and smart contract development.
    * **contracts/:** Houses all smart contract examples.
        * **Difficulty Levels:**
            * **learn/:** Easy-to-understand contracts for beginners.
            * **practice/:** Contracts with slightly more complex functionalities.
            * **master/:** Contracts with advanced features to master the craft.
        * **Contract Structure:**
            * Each difficulty level folder holds individual contract sub-folders.
            * Each contract subfolder maintains the same structure:
                * **Cargo.toml:** Project configuration file.
                * **src/:** Contains the contract's source code (`lib.rs`).
                * **tests/:** (Optional) Unit tests for the contract.

**How to Use:**

1. **Choose a Contract Example:** Select a contract from the appropriate difficulty level (learn, practice, master).
2. **Clone the Repository:** Use Git to clone this repository to your local machine.
3. **Navigate to Contract Folder:** Open a terminal and navigate to the specific contract's folder.
4. **Run the Contract (Optional):** Some contracts may include instructions in a `README.md` file within their folder for compilation and deployment (if applicable).
5. **Learn & Experiment:** Explore the code, understand the logic, and try making modifications to solidify your understanding.

**Contributing:**

We welcome contributions to this repository! If you'd like to add new smart contract examples or improve existing ones, please refer to a `CONTRIBUTING.md` file for guidelines.

**License:**

This repository is licensed under the MIT license. See the `LICENSE` file for details.

**Community Resources:**

For further learning and discussion, consider joining the NEAR community:

- NEAR Docs: [https://docs.near.org/](https://docs.near.org/)
- NEAR Discord: [https://discord.com/invite/zfhfRpaM4m](https://discord.com/invite/zfhfRpaM4m)
- NEAR Nomicon: [https://nomicon.io/](https://nomicon.io/)

**Additional Notes:**

This repository is under active development. Feel free to report any issues or suggest improvements through GitHub issues.

**Happy Learning!**
