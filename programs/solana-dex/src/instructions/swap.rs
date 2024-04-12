use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_lang::solana_program::program::{invoke, invoke_signed};
use anchor_lang::solana_program::system_instruction;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::errors::CustomError;
use crate::state::*;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct SwapToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>, // Pool's token account
    #[account(mut)]
    pub sol_account: Account<'info, TokenAccount>, // Pool's SOL account
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler_swap_sol_to_token(ctx: Context<SwapToken>, sol_amount: u64) -> Result<()> {
    let x = ctx.accounts.token_account.amount;
    let y = ctx.accounts.sol_account.amount;
    let k = x * y;

    let delta_y = sol_amount;
    let delta_x = k.checked_div(y.checked_add(delta_y).ok_or(CustomError::Overflow)?)
        .ok_or(CustomError::DivisionError)?
        .checked_sub(x)
        .ok_or(CustomError::Underflow)?;

    ctx.accounts.transfer_sol_to_pool(delta_y)?;
    ctx.accounts.transfer_token_to_user(delta_x)?;
    Ok(())
}

pub fn handler_swap_token_to_sol(ctx: Context<SwapToken>, token_amount: u64) -> Result<()> {
    let x = ctx.accounts.token_account.amount;
    let y = ctx.accounts.sol_account.amount;
    let k = x * y;

    let delta_x = token_amount;
    let delta_y = k.checked_div(x.checked_add(delta_x).ok_or(CustomError::Overflow)?)
        .ok_or(CustomError::DivisionError)?
        .checked_sub(y)
        .ok_or(CustomError::Underflow)?;

    ctx.accounts.transfer_token_to_pool(delta_x)?;
    ctx.accounts.transfer_sol_to_user(delta_y)?;
    Ok(())
}

impl<'info> SwapToken<'info> {
    fn transfer_sol_to_pool(&self, lamports_amount: u64) -> Result<()> {
        let transfer_instruction = system_instruction::transfer(
            &self.user.key(),
            &self.sol_account.key(),
            lamports_amount,
        );

        invoke(
            &transfer_instruction,
            &[
                self.user.to_account_info(),
                self.sol_account.to_account_info(),
                self.system_program.to_account_info(),
            ],
        )?;

        Ok(())
    }

    fn transfer_token_to_user(&self, token_amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.token_account.to_account_info(),
            to: self.user.to_account_info(),
            authority: self.token_account.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, token_amount)?;

        Ok(())
    }

    fn transfer_token_to_pool(&self, token_amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.token_account.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, token_amount)?;

        Ok(())
    }

    fn transfer_sol_to_user(&self, lamports_amount: u64) -> Result<()> {
        let ix = system_instruction::transfer(
            &self.sol_account.key(),
            &self.user.key(),
            lamports_amount,
        );

        invoke(
            &ix,
            &[
                self.sol_account.to_account_info(),
                self.user.to_account_info(),
                self.system_program.to_account_info(),
            ],
        )?;

        Ok(())
    }
}
