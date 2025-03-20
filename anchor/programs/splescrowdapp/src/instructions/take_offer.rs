use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::OfferState;

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

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
        has_one=token_mint_a,
        has_one=token_mint_b,
        constraint = maker.key() == offer.maker,
        close=maker,
        seeds=[
            b"offer",
            maker.key().as_ref(),
            offer.seed.to_le_bytes().as_ref(),
        ],
        bump=offer.bump
    )]
    pub offer: Account<'info, OfferState>,

    #[account(
        mut,
        associated_token::authority=offer,
        associated_token::mint=token_mint_a,
        associated_token::token_program=token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
            init_if_needed,
            payer=taker,
            associated_token::mint=token_mint_b,
            associated_token::authority=taker,
            associated_token::token_program=token_program,
        )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint=token_mint_b,
        associated_token::authority=taker,
        associated_token::token_program=token_program,
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint=token_mint_b,
        associated_token::authority=maker,
        associated_token::token_program=token_program,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
