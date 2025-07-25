// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/// @title SimpleToken
/// @notice A minimal ERC-20-like token with a reentrancy-vulnerable withdraw function
contract SimpleToken {
    string public name = "SimpleToken";
    string public symbol = "STK";
    uint8 public decimals = 18;
    mapping(address => uint256) public balance;

    /// @notice Mint new tokens to an address
    function mint(address to, uint256 amount) external {
        balance[to] += amount;
    }

    /// @notice Withdraw your tokens (vulnerable if you send Ether)
    function withdraw(uint256 amount) external {
        require(balance[msg.sender] >= amount, "Insufficient balance");
        // Vulnerable pattern: external call before balance update
        (bool success, ) = msg.sender.call{value: amount}("");
        require(success, "Transfer failed");
        balance[msg.sender] -= amount;
    }

    // Fallback to accept Ether for withdraw demo
    receive() external payable {}
}
