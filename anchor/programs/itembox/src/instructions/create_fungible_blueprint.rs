use anchor_lang::{prelude::*, solana_program::entrypoint::ProgramResult};
use anchor_spl::token_interface::{
  token_metadata_initialize, Mint,
  Token2022, TokenMetadataInitialize,
};

use crate::{
  get_meta_list_size, states::{Blueprint, Main, Profile}, update_account_lamports_to_minimum_balance, META_LIST_ACCOUNT_SEED
};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateFungibleBlueprintArgs {
  name: String,
  uri: String,
  symbol: String,
  treasury: Pubkey,
  mint_authority: Pubkey,
  publish: bool
}

#[derive(Accounts)]
#[instruction(args: CreateFungibleBlueprintArgs)]
pub struct CreateFungibleBlueprint<'info> {

  #[account(
    init, 
    payer = owner, 
    seeds = [
      b"blueprint",
      mint.key().as_ref()
    ], 
    bump, 
    space = 8 + Blueprint::INIT_SPACE
  )]
  pub blueprint: Box<Account<'info, Blueprint>>,

  #[account(mut)]
  /// CHECK: has_one in the main account
  pub treasury: UncheckedAccount<'info>,

  #[account(
    mut,
    seeds = [
      b"main"
    ],
    bump = main.bump,
    has_one = treasury,
  )]
  pub main: Box<Account<'info, Main>>,

  #[account(
    init,
    signer,
    payer = owner,
    mint::token_program = token_program,
    mint::decimals = 0,
    mint::authority = main,
    mint::freeze_authority = main,
    extensions::metadata_pointer::authority = main,
    extensions::metadata_pointer::metadata_address = mint,
    // extensions::group_member_pointer::authority = main,
    // extensions::group_member_pointer::member_address = mint,
    // extensions::transfer_hook::authority = main,
    // extensions::transfer_hook::program_id = crate::ID,
    // extensions::close_authority::authority = main,
    // extensions::permanent_delegate::delegate = main,
  )]
  pub mint: Box<InterfaceAccount<'info, Mint>>,

  #[account(
    seeds = [b"profile", owner.key().as_ref()],
    bump = profile.bump
  )]
  pub profile: Option<Box<Account<'info, Profile>>>,

  #[account(mut)]
  pub owner: Signer<'info>,

  pub system_program: Program<'info, System>,
  /// CHECK: This account's data is a buffer of TLV data
  #[account(
    init,
    space = get_meta_list_size(None),
    seeds = [META_LIST_ACCOUNT_SEED, mint.key().as_ref()],
    bump,
    payer = owner,
  )]
  pub extra_metas_account: UncheckedAccount<'info>,
  pub token_program: Program<'info, Token2022>,
}

// TODO: include verifier, profile account
pub fn create_fungible_blueprint_handler(
  ctx: Context<CreateFungibleBlueprint>,
  args: CreateFungibleBlueprintArgs
) -> Result<()> {

  let blueprint = &mut ctx.accounts.blueprint;
  let treasury = &mut ctx.accounts.treasury;
  let owner = &ctx.accounts.owner;
  let profile = &ctx.accounts.profile;
  let mint_fee = ctx.accounts.main.blueprint_creation_fee;

  // pay fee to treasury

  let ix = anchor_lang::solana_program::system_instruction::transfer(
    &owner.key(),
    &treasury.key(),
    mint_fee,
  );
  anchor_lang::solana_program::program::invoke(
    &ix,
    &[
      owner.to_account_info(),
      treasury.to_account_info(),
      ctx.accounts.system_program.to_account_info(),
    ],
  )?;

  blueprint.bump = ctx.bumps.blueprint;
  blueprint.mint = ctx.accounts.mint.key();
  blueprint.non_fungible = false;
  blueprint.authority = ctx.accounts.owner.key();
  blueprint.treasury = args.treasury.key();
  blueprint.mint_authority = args.mint_authority.key();
  blueprint.counter = 0;
  blueprint.published = if args.publish { 1 } else { 0 };
  blueprint.status = profile
    .as_ref()
    .map(|p| if p.status == 1 { 3 } else { 0 })
    .unwrap_or(0);

  ctx.accounts.initialize_token_metadata(
    args.name.clone(),
    args.symbol.clone(),
    args.uri.clone(),
  )?;

  ctx.accounts.mint.reload()?;

  update_account_lamports_to_minimum_balance(
    ctx.accounts.mint.to_account_info(),
    ctx.accounts.owner.to_account_info(),
    ctx.accounts.system_program.to_account_info(),
  )?;

  Ok(())
}

impl<'info> CreateFungibleBlueprint<'info> {
  fn initialize_token_metadata(
    &self,
    name: String,
    symbol: String,
    uri: String,
  ) -> ProgramResult {
    let cpi_accounts = TokenMetadataInitialize {
      token_program_id: self.token_program.to_account_info(),
      mint: self.mint.to_account_info(),
      metadata: self.mint.to_account_info(), // metadata account is the mint, since data is stored in mint
      mint_authority: self.main.to_account_info(),
      update_authority: self.main.to_account_info(),
    };
    let bump = self.main.bump;
    let additional_seeds: &[&[&[u8]]] = &[&[b"main", &[bump]]];

    let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
      .with_signer(additional_seeds);

    token_metadata_initialize(cpi_ctx, name, symbol, uri)?;
    Ok(())
  }
}