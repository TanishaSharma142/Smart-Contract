# SOLANA_VAULT (SOL)

## Project Overview
This project implements a secure **Token Vault** on the Solana Blockchain using the Anchor Framework. It demonstrates fundamental Web3 concepts including State Initialization, Controlled Mutation, and Role-Based Access Control (RBAC).

**Core Functionality:**
* Initialize: A user creates a personal vault. The program creates a dedicated on-chain wallet (PDA) for them.
* Deposit: Any user can deposit Native SOL into this vault.
* Withdraw: Only the original creator (Owner) can withdraw funds.

## Design Choices
* Anchor Framework: Used for its rigorous compile-time checks and simplified account deserialization, reducing the risk of common Solana security pitfalls.
* Native SOL: Chosen over SPL tokens to keep the logic focused on account ownership and state management without external dependencies.
* PDAs (Program Derived Addresses): Used for the "Vault Wallet". This ensures that the Private Key for the funds effectively *does not exist*. Only the Program itself can authorize fund movement, mathematically guaranteeing safety.

## State Machine & Sequence
1.  **State Initialization (`initialize`)**
    * User calls program -> Program derives `VaultState` address (using seeds) -> Program saves User's Public Key inside `VaultState`.
2.  **Controlled Mutation (`deposit`)**
    * User signs tx -> Program verifies `VaultState` exists -> Program invokes System Transfer -> Funds move to `VaultAuth` PDA.
3.  **Access Control (`withdraw`)**
    * User signs tx -> Program checks `User Key == Saved Owner Key` -> If Match: Program subtracts lamports from PDA and adds to User.

## Security Checks
* Signer Validation: The `initialize` and `withdraw` functions enforce that the relevant accounts have signed the transaction.
* Owner Validation: Explicit constraint `require!(owner.key() == vault_state.owner)` prevents unauthorized withdrawals.
* Seed Constraints: The `#[account(seeds = ...)]` macros ensure that users cannot inject fake accounts. The program will strictly reject any account that is not the legitimate, mathematically derived PDA.

## Deployed Link
* Network: Solana Devnet
* Program ID: `BYQXDGdYAR2zZ2u2Nio4eUjBwaVbobMgwhLsB8CZwg8d`
* **Explorer Link:** https://explorer.solana.com/address/BYQXDGdYAR2zZ2u2Nio4eUjBwaVbobMgwhLsB8CZwg8d?cluster=devnet

