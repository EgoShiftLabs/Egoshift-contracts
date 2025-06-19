// SPDX-License-Identifier: MIT
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("SHIFT111111111111111111111111111111111111111");

#[program]
pub mod egoshift_contracts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let stake_data = &mut ctx.accounts.stake_data;
        let clock = Clock::get()?;

        stake_data.owner = *ctx.accounts.user.key;
        stake_data.amount += amount;
        stake_data.start_time = clock.unix_timestamp;

        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.vault_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        let stake_data = &mut ctx.accounts.stake_data;
        let clock = Clock::get()?;
        let duration = clock.unix_timestamp - stake_data.start_time;
        let xp_earned = stake_data.amount.checked_mul(duration as u64).unwrap_or(0);

        stake_data.amount = 0;
        stake_data.start_time = 0;

        msg!("XP earned: {}", xp_earned);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault_account: Account<'info, TokenAccount>,
    #[account(init_if_needed, payer = user, space = 8 + 40)]
    pub stake_data: Account<'info, StakeData>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub stake_data: Account<'info, StakeData>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct StakeData {
    pub owner: Pubkey,
    pub amount: u64,
    pub start_time: i64,
}
