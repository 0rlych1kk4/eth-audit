# Ethereum Smart Contract Audit Lab

This repository contains modular audit simulations and CLI tools for detecting common Ethereum smart contract vulnerabilities. It is designed for educational, research, and auditing purposes, highlighting critical security patterns and fixes.

##  Audit Modules

Each module demonstrates a specific vulnerability using Rust and simulated smart contracts:

- `eth_audit_reentrancy` – Reentrancy attack detection and simulation  
- `eth_audit_unchecked_send` – Unchecked `send()` usage and exploits  
- `eth_audit_integer_overflow` – Integer overflow/underflow audit  
- `eth_audit_core` – Core shared utilities for auditing modules  
- `contracts/` – Sample Solidity contracts for testing

##  CLI Tool

A standalone command-line tool is available in the [`eth-audit-cli`](./eth-audit-cli) submodule:

```bash
cd eth-audit-cli
cargo run -- scan ./contracts/MyContract.sol
```
##  Getting Started

### Clone and build:

git clone --recurse-submodules https://github.com/0rlych1kk4/eth-audit.git
cd eth-audit
cargo build

### Run a test module:

cargo test --package eth_audit_reentrancy

### Structure

eth-audit/
├── eth_audit_reentrancy/
├── eth_audit_unchecked_send/
├── eth_audit_integer_overflow/
├── eth_audit_core/
├── eth-audit-cli/ (submodule)
└── contracts/

##  License

MIT © Orly Trajano – For research and educational use. Not for production audits without review.


---
