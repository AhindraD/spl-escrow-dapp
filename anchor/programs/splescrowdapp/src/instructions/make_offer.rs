use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{constants::ANCHOR_DISCRIMINATOR, OfferState};

#[derive(Accounts)]
#[instruction(id:u64)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mint::token_program=token_program
    )]
    pub token_mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program=token_program
    )]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint=token_mint_a,
        associated_token::authority=maker,
        associated_token::token_program=token_program,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer=maker,
        seeds=[
            b"offer",
            maker.key().as_ref(),
            id.to_le_bytes().as_ref(),
        ],
        space= ANCHOR_DISCRIMINATOR as usize + OfferState::INIT_SPACE,
        bump,
    )]
    pub offer: Account<'info, OfferState>,

    #[account(
        init,
        payer=maker,
        associated_token::authority=offer,
        associated_token::mint=token_mint_a,
        associated_token::token_program=token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
