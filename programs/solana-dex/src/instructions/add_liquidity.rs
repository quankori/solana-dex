use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, Token, TokenAccount, Transfer };

use crate::state::*;
use crate::errors::CustomError;
use crate::constants::{ CONSTANT_CONFIG_ACCOUNT_SEED, CONSTANT_TOKEN_ACCOUNT_SEED };

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct AddLiquidity<'info> {
    // Pair account
    #[account(
        mut,
        seeds = [
            CONSTANT_TOKEN_ACCOUNT_SEED,
            authority.key().as_ref(),
            token_mint_address.key().as_ref(),
            pair_metadata.key().as_ref(),
        ],
        bump = pair_metadata.pair_token_account_bump,
        token::authority = pair_metadata
    )]
    pub pair_token_account: Account<'info, TokenAccount>,

    // Config account
    #[account(
        seeds = [
            CONSTANT_CONFIG_ACCOUNT_SEED,
            authority.key().as_ref(),
            token_mint_address.key().as_ref(),
        ],
        bump = pair_metadata.pair_config_account_bump
    )]
    pub pair_metadata: Account<'info, PairMetadataAccount>,

    pub token_mint_address: Account<'info, Mint>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub authority: AccountInfo<'info>,

    // Deposit account
    #[account(mut,
        token::mint=token_mint_address,
        token::authority = depositor,
        constraint = depositor_token_account.amount >= amount @ CustomError::InsufficientFunds
    )]
    pub depositor_token_account: Account<'info, TokenAccount>,

    // Signer
    #[account(mut, constraint = depositor.lamports() > 0 && depositor.data_is_empty() @ CustomError::InsufficientFunds)]
    pub depositor: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler_add_liquidity<'info>(
    ctx: Context<'_, '_, '_, 'info, AddLiquidity<'info>>,
    amount: u64
) -> Result<()> {
    let sender = ctx.accounts.depositor_token_account.to_account_info();
    let receiver = ctx.accounts.pair_token_account.to_account_info();
    let authority = ctx.accounts.depositor.to_account_info();
    let token_program = ctx.accounts.token_program.to_account_info();
    let transfer_account = Transfer {
        from: sender.to_account_info(),
        to: receiver.to_account_info(),
        authority: authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(token_program.to_account_info(), transfer_account);
    anchor_spl::token::transfer(cpi_ctx, amount)?;
    Ok(())
}
