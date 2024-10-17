use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GameInfo {

  /// Owner of this game info. (32)
  pub authority: Pubkey,

  /// Status of the game: Unpublished [0], Published [1], Banned [2]. (1)
  pub status: u8,

  /// External JSON details of the game. (200)
  #[max_len(200)]
  pub uri: String
}