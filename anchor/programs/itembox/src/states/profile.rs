use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Profile {
  /// Bump nonce of the PDA. (1)
  pub bump: u8,

  /// Owner of this profile (seeds = ["profile", authority]). (32)
  pub authority: Pubkey,

  /// Status of the profile: On Review [0], Published [1], Banned [2]. (1)
  pub status: u8,

  /// External JSON details of the profile. (200)
  #[max_len(200)]
  pub uri: String
}