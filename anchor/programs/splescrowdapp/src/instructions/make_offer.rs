use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
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

impl<'info> MakeOffer<'info> {
    pub fn send_offered_tokens_to_vault(&mut self, deposit: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_account_options = TransferChecked {
            from: self.maker_ata_a.to_account_info(),
            mint: self.token_mint_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_account_options);
        transfer_checked(cpi_ctx, deposit, self.token_mint_a.decimals)?;
        Ok(())
    }

    pub fn save_offer(
        &mut self,
        id: u64,
        offered_amount: u64,
        asked_amount: u64,
        bumps: &MakeOfferBumps,
    ) -> Result<()> {
        self.offer.set_inner(OfferState {
            maker: self.maker.key(),
            token_mint_a: self.token_mint_a.key(),
            token_mint_b: self.token_mint_a.key(),
            token_a_offered: offered_amount,
            token_b_wanted: asked_amount,
            seed: id,
            bump: bumps.offer,
        });
        Ok(())
    }
}
