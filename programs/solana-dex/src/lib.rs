use anchor_lang::prelude::*;
use crate::instructions::*;
pub mod constants;
pub mod instructions;
pub mod state;
pub mod errors;

declare_id!("p7naqtynAgMUMWiD6DvG8yYKbuBpGKsn8HtHGn1mT67");

#[program]
pub mod solana_dex {
    use super::*;

    pub fn add_liquidity<'info>(
        ctx: Context<'_, '_, '_, 'info, AddLiquidity<'info>>,
        amount: u64
    ) -> Result<()> {
        handler_add_liquidity(ctx, amount)?;
        Ok(())
    }

    pub fn swap_token_to_sol<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapToken<'info>>,
        amount: u64
    ) -> Result<()> {
        handler_swap_token_to_sol(ctx, amount)?;
        Ok(())
    }

    pub fn swap_sol_to_token<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapToken<'info>>,
        amount: u64
    ) -> Result<()> {
        handler_swap_sol_to_token(ctx, amount)?;
        Ok(())
    }
}
