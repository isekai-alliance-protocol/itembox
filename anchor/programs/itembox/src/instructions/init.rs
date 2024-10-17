use anchor_lang::prelude::*;

use crate::states::Main;

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InitArgs {
  pub treasury: Pubkey,
  pub blueprint_verifier: Pubkey,
  pub profile_verifier: Pubkey,
  pub token_mint: Pubkey,
  pub blueprint_creation_fee: u64,
  pub recipe_creation_fee: u64,
  pub profile_registration_fee: u64,
}

#[derive(Accounts)]
#[instruction(args: InitArgs)]
pub struct Init<'info> {
  #[account(
    init, 
    payer = authority, 
    seeds = [
      b"main",
    ], 
    bump, 
    space = 8 + Main::INIT_SPACE
  )]
  pub main: Box<Account<'info, Main>>,

  #[account(mut)]
  pub authority: Signer<'info>,

  pub system_program: Program<'info, System>,
}

pub fn init_handler(ctx: Context<Init>, args: InitArgs) -> Result<()> {
  let main = &mut ctx.accounts.main;

  main.bump = ctx.bumps.main;
  main.authority = ctx.accounts.authority.key();
  main.blueprint_verifier = args.blueprint_verifier;
  main.profile_verifier = args.profile_verifier;
  main.treasury = args.treasury.key();
  main.token_mint = args.token_mint.key();
  main.blueprint_creation_fee = args.blueprint_creation_fee;
  main.recipe_creation_fee = args.recipe_creation_fee;
  main.profile_registration_fee = args.profile_registration_fee;
  
  Ok(())
}
