#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod constants;
pub mod error;

pub mod state;
pub use state::*;

pub mod instructions;
pub use instructions::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod splescrowdapp {
    use super::*;

    pub fn make_offer(
        ctx: Context<MakeOffer>,
        id: u64,
        offered_amount: u64,
        asked_amount: u64,
    ) -> Result<()> {
        MakeOffer::send_offered_tokens_to_vault(ctx.accounts, offered_amount)?;
        MakeOffer::save_offer(ctx.accounts, id, offered_amount, asked_amount, &ctx.bumps)
    }
}
