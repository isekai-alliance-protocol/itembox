use anchor_lang::prelude::*;

#[account]
pub struct Main {
  /// Bump nonce of the PDA. (1)
  pub bump: u8,

  /// The authority that is permitted to update this state. (32)
  pub authority: Pubkey,

  /// The authority that approves the blueprint to be displayed on the "Explore Blueprints" page. (32)
  pub blueprint_verifier: Pubkey,

  /// The authority that approves the created profile. (32)
  pub profile_verifier: Pubkey,

  /// Governance token of Itembox. (32)
  pub token_mint: Pubkey,

  /// The wallet that stores the collected fees. (32)
  pub treasury: Pubkey,

  /// Amount of fee being collected when minting a blueprint. (8)
  pub blueprint_creation_fee: u64,

  /// Amount of fee being collected when minting a recipe. (8)
  pub recipe_creation_fee: u64,

  /// Fee for profile registration. (8)
  pub profile_registration_fee: u64,

  /// Unused reserved byte space for future additive changes. (128)
  pub _reserved: [u8; 128],
}

impl Main {
  pub fn len() -> usize {
    8 + 1 + 32 + 32 + 32 + 32 + 32 + 8 + 8 + 8 + 128
  }
}
