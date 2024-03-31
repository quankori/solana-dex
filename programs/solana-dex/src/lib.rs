use anchor_lang::prelude::*;

declare_id!("p7naqtynAgMUMWiD6DvG8yYKbuBpGKsn8HtHGn1mT67");

#[program]
pub mod solana_dex {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
