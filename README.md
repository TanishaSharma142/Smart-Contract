# Solana Vault (SOL)

## Contract Summary
This project implements a secure **Native SOL Vault** on the Solana Blockchain using the Anchor Framework. It demonstrates the "Treasury" pattern where a program acts as a custodian of funds.
* **Custody:** Funds are held in a Program Derived Address (PDA), ensuring no private key exists that could be compromised.
* **Access Control:** Deposits are open to the public, but withdrawals are strictly restricted to the vault's original creator (Owner).

## How to Build & Test
**Prerequisites:** Rust, Solana CLI, Anchor Framework.

1.  **Build the program:**
    ```bash
    anchor build
    ```

2.  **Run the test suite (Integration Tests):**
    ```bash
    anchor test
    ```
    *Tests cover: Initialization, Depositing funds, and Verifying balance updates.*

*Note: If using Solana Playground (Solpg), use the `build` button and run the client script via the `run` command.*

## State & Flows

### 1. Initialization (`initialize`)
* **Input:** User's Wallet.
* **Action:**
    * Derives a `VaultState` account using seeds `["state", owner_key]`.
    * Derives a `VaultAuth` PDA using seeds `["auth", state_key]`.
    * Saves the `owner` public key in `VaultState` for future permission checks.

### 2. Deposit (`deposit`)
* **Input:** Amount (in lamports).
* **Flow:** User signs transaction -> Program invokes System Transfer -> Funds move from User to `VaultAuth` PDA.

### 3. Withdraw (`withdraw`)
* **Input:** Amount (in lamports).
* **Flow:**
    * User signs transaction.
    * **Check:** Program verifies `Signer Key == Saved Owner Key`.
    * **Action:** If authorized, Program subtracts lamports from `VaultAuth` and adds them to the Owner's account.

## Known Limitations
* **Asset Support:** Currently supports Native SOL only. It does not support SPL Tokens (like USDC).
* **Single Instance:** The seed derivation uses the owner's public key directly (`["state", owner_key]`), limiting each wallet address to creating exactly one vault.

## Deployed Link
* **Network:** Solana Devnet
* **Program ID:** `BYQXDGdYAR2zZ2u2Nio4eUjBwaVbobMgwhLsB8CZwg8d`
* **Explorer Link:** https://explorer.solana.com/address/BYQXDGdYAR2zZ2u2Nio4eUjBwaVbobMgwhLsB8CZwg8d?cluster=devnet
