import { BN, Program, Provider, setProvider } from '@coral-xyz/anchor'
import { Itembox } from '../target/types/itembox'
import idl from '../target/idl/itembox.json'
import { LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js'

module.exports = async function (provider: Provider) {
  setProvider(provider)

  const itemBoxIdl = idl as Itembox
  const program = new Program<Itembox>(itemBoxIdl, provider)

  const [mainPda] = PublicKey.findProgramAddressSync(
    [Buffer.from('main')],
    program.programId
  )

  console.log('Main PDA address', mainPda.toBase58())

  try {
    await program.methods
      .init({
        blueprintMintFee: new BN(0.0002 * LAMPORTS_PER_SOL),
        tokenMint: new PublicKey(
          'DQTNP5FBEcCsEkdPH5JQfAjJAyAavbCHfhk2T5YAUj4L'
        ),
        treasury: provider.publicKey,
      })
      .accounts({
        authority: provider.publicKey,
      })
      .rpc()
  } catch (e) {
    console.error(e)
  }
}
