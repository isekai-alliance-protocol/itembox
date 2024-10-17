use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GameInfo {
  /// Bump nonce of the PDA. (1)
  pub bump: u8,

  /// Owner of this game info. (32)
  pub authority: Pubkey,

  /// Status of the game: Unpublished [0], Published [1], Banned [2]. (1)
  pub status: u8,

  /// External JSON details of the game. (200)
  #[max_len(200)]
  pub uri: String
}