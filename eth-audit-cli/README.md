# eth-audit-cli

 **Command-Line Interface for Ethereum Smart Contract Security Audits**

Part of the [`eth-audit`](https://github.com/0rlych1kk4/eth-audit) suite, this CLI tool provides developers and auditors with an accessible interface to scan, simulate, and document common Ethereum smart contract vulnerabilities.

##  Features

-  Static analysis of Solidity contracts  
-  Detects reentrancy, unchecked calls, integer overflows, and more  
-  Runs pre-defined attack simulations  
-  Outputs findings in JSON format for integration/reporting  
- ️ Designed to be modular and developer-friendly

##  Installation

```bash
git clone --recurse-submodules https://github.com/0rlych1kk4/eth-audit.git
cd eth-audit/eth-audit-cli
cargo build --release
```
##  Usage

./target/release/eth-audit-cli --scan contracts/MyContract.sol

###  Options
Flag	Description
--scan <file>	Scan a Solidity file for known patterns
--simulate <attack>	Run specific exploit simulations
--export <file>	Export findings to a JSON file

##  Project Structure

eth-audit/
├── eth-audit-cli/              # CLI Tool (this repo)
├── eth_audit_core/             # Core library for audit rules
├── eth_audit_reentrancy/       # Reentrancy simulations
├── eth_audit_unchecked_send/   # Unchecked `send()` simulations
├── eth_audit_integer_overflow/ # Arithmetic bug simulations
├── contracts/                  # Test and sample contracts
└── findings.json               # Example output from CLI

##  License

This project is licensed under the MIT License. See LICENSE for more details.

##  Contributing

Pull requests and issues welcome! If you've simulated a new class of vulnerability, feel free to open a discussion or contribute a module.

##  Maintainer

Orly Trajano – GitHub
