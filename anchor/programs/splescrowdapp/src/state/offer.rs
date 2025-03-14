use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct OfferState {
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_a_offered: u64,
    pub token_b_wanted: u64,
    pub seed: u64,
    pub bump: u8,
}
