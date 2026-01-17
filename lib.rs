use anchor_lang::prelude::*;
use anchor_lang::system_program;
// program_id
// This ID is a placeholder. Solpg will update it automatically when you build.
declare_id!("BYQXDGdYAR2zZ2u2Nio4eUjBwaVbobMgwhLsB8CZwg8d");

#[program]
pub mod solana_vault {
    use super::*;

    // 1. INITIALIZE VAULT (Sets up the Vault State and the Vault Wallet (PDA))
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault_state = &mut ctx.accounts.vault_state;

        // Ownership: Set the signer as the permanent owner
        vault_state.owner = ctx.accounts.owner.key();
        msg!("Vault Initialized! Owner: {}", vault_state.owner);
        Ok(())
    }

    // 2. DEPOSIT (Anyone can deposit SOL into the Vault's PDA.)
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {

        // Check: Ensure strictly positive deposit
        require!(amount > 0, VaultError::InvalidAmount);

        // CPI (Cross-Program Invocation) to System Program
        // We are "invoking" the System Program to move money from User -> Vault PDA
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.vault_auth.to_account_info(),
            },
        );
        system_program::transfer(cpi_context, amount)?;
        msg!("Deposited {} lamports", amount);
        Ok(())
    }

    // 3. WITHDRAW (Only Owner i.e. Securely transfers SOL from Vault PDA -> Owner.)

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let vault_state = &ctx.accounts.vault_state;
        let vault_auth = &mut ctx.accounts.vault_auth;
        let owner = &mut ctx.accounts.owner;

        // Check 1: Is the signer the owner?
        require!(owner.key() == vault_state.owner, VaultError::Unauthorized);

        // Check 2: Does vault have enough funds?
        require!(vault_auth.lamports() >= amount, VaultError::InsufficientFunds);

        // Transfer from Vault PDA to Owner
        // Note: PDAs cannot "sign" a CPI like a user. 
        // We modify lamports directly because our Program "owns" the PDA account.
        **vault_auth.try_borrow_mut_lamports()? -= amount;
        **owner.try_borrow_mut_lamports()? += amount;

        msg!("Withdrawn {} lamports", amount);
        Ok(())
    }
}

// --- ACCOUNT VALIDATION STRUCTURES ---

// 1. Vault State: Stores data (Owner address). Derived from "state" + Owner ID.
#[account]
pub struct VaultState {
    pub owner: Pubkey,
}

// 2. Vault Auth: The "Wallet" that holds the SOL. Derived from "auth" + State Key.
// CHECK: Safe because we derive it deterministically via seeds.
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = owner, 
        space = 8 + 32, // Discriminator + Pubkey
        seeds = [b"state", owner.key().as_ref()], 
        bump
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        seeds = [b"auth", vault_state.key().as_ref()],
        bump
    )]
    pub vault_auth: SystemAccount<'info>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        seeds = [b"state", vault_state.owner.as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(mut, seeds = [b"auth", vault_state.key().as_ref()], bump)]
    pub vault_auth: SystemAccount<'info>,
    #[account(mut)]
    pub user: Signer<'info>, // Any user
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        seeds = [b"state", vault_state.owner.as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(mut, seeds = [b"auth", vault_state.key().as_ref()], bump)]
    pub vault_auth: SystemAccount<'info>,
    #[account(mut)]
    pub owner: Signer<'info>, // Must be owner
}

#[error_code]
pub enum VaultError {
    #[msg("You are not authorized to withdraw.")]
    Unauthorized,
    #[msg("Not enough funds in vault.")]
    InsufficientFunds,
}
