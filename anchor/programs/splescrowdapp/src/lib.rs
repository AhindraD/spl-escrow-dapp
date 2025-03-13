#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod splescrowdapp {
    use super::*;

  pub fn close(_ctx: Context<CloseSplescrowdapp>) -> Result<()> {
    Ok(())
  }

  pub fn decrement(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.splescrowdapp.count = ctx.accounts.splescrowdapp.count.checked_sub(1).unwrap();
    Ok(())
  }

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.splescrowdapp.count = ctx.accounts.splescrowdapp.count.checked_add(1).unwrap();
    Ok(())
  }

  pub fn initialize(_ctx: Context<InitializeSplescrowdapp>) -> Result<()> {
    Ok(())
  }

  pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
    ctx.accounts.splescrowdapp.count = value.clone();
    Ok(())
  }
}

#[derive(Accounts)]
pub struct InitializeSplescrowdapp<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  init,
  space = 8 + Splescrowdapp::INIT_SPACE,
  payer = payer
  )]
  pub splescrowdapp: Account<'info, Splescrowdapp>,
  pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseSplescrowdapp<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  mut,
  close = payer, // close account and return lamports to payer
  )]
  pub splescrowdapp: Account<'info, Splescrowdapp>,
}

#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub splescrowdapp: Account<'info, Splescrowdapp>,
}

#[account]
#[derive(InitSpace)]
pub struct Splescrowdapp {
  count: u8,
}
