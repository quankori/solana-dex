use anchor_lang::prelude::*;

#[account]
pub struct PairMetadataAccount {
    pub pair_config_account_bump: u8,
    pub pair_token_account_bump: u8,
    pub pair_native_account_bump: u8,
    pub token_price: u64,
    pub is_active: bool,
    pub token_mint_address: Pubkey,
    pub pair_token_account: Pubkey,
    pub pair_native_account: Pubkey,
    pub authority: Pubkey,
}
