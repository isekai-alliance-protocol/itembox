use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Blueprint {
  /// Bump nonce of the PDA. (1)
  pub bump: u8,

  /// Status that tells that the Blueprint is okay to be displayed in "Explore Blueprints" page. 
  /// Pending [0], Verified [1], Banned [2], Pre-verified [3], Pending Integration [4], Approved & Integrated [5]. (1)
  pub status: u8,

  /// Status that tells that the Blueprint is ready to be used and displayed. 
  /// Can be toggled off by the author if counter == 0. (1)
  /// Unpublished [0], Published [1]
  pub published: u8,

  /// The address of the metaplex core collection with master edition plugin OR
  /// the mint address of the fungible token. (32)
  pub mint: Pubkey,

  /// Whether the item is non-fungible. (1)
  pub non_fungible: bool,

  /// The creator and owner of this blueprint, 
  /// which also acts as the update authority. (32)
  pub authority: Pubkey,

  /// The receiver of the transfered items if the Recipe is configured 
  /// to transfer an ingredient. (32)
  pub treasury: Pubkey,

  /// The account who can mint the item of this blueprint. 
  /// Note: Recipes are still able to MINT this item if the condition is met. (32)
  pub mint_authority: Pubkey,

  /// Number of editions printed, if mint is a Master Edition. (8)
  pub counter: u32,

  /// Unused reserved byte space for future additive changes. (128)
  pub _reserved: [u8; 128],
}

impl Blueprint {
  pub fn from_account_info(account_info: &AccountInfo) -> Result<Self> {
    // Borrow data from the account_info and skip the first 8 bytes (the discriminator)
    let borrowed_data = &account_info.try_borrow_data()?[8..];

    // Attempt to deserialize the remaining data into Blueprint struct
    Blueprint::try_from_slice(borrowed_data).map_err(|_| ProgramError::InvalidAccountData.into())
  }
}

// TODO: ASSET ADAPTER
// - abstracts common CPI calls, burn, transfer, etc
// - abstract any information for metadata, etc.