import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { Keypair } from '@solana/web3.js'
import { Splescrowdapp } from '../target/types/splescrowdapp'

describe('splescrowdapp', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const payer = provider.wallet as anchor.Wallet

  const program = anchor.workspace.Splescrowdapp as Program<Splescrowdapp>

  const splescrowdappKeypair = Keypair.generate()

  it('Initialize Splescrowdapp', async () => {
    await program.methods
      .initialize()
      .accounts({
        splescrowdapp: splescrowdappKeypair.publicKey,
        payer: payer.publicKey,
      })
      .signers([splescrowdappKeypair])
      .rpc()

    const currentCount = await program.account.splescrowdapp.fetch(splescrowdappKeypair.publicKey)

    expect(currentCount.count).toEqual(0)
  })

  it('Increment Splescrowdapp', async () => {
    await program.methods.increment().accounts({ splescrowdapp: splescrowdappKeypair.publicKey }).rpc()

    const currentCount = await program.account.splescrowdapp.fetch(splescrowdappKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Increment Splescrowdapp Again', async () => {
    await program.methods.increment().accounts({ splescrowdapp: splescrowdappKeypair.publicKey }).rpc()

    const currentCount = await program.account.splescrowdapp.fetch(splescrowdappKeypair.publicKey)

    expect(currentCount.count).toEqual(2)
  })

  it('Decrement Splescrowdapp', async () => {
    await program.methods.decrement().accounts({ splescrowdapp: splescrowdappKeypair.publicKey }).rpc()

    const currentCount = await program.account.splescrowdapp.fetch(splescrowdappKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Set splescrowdapp value', async () => {
    await program.methods.set(42).accounts({ splescrowdapp: splescrowdappKeypair.publicKey }).rpc()

    const currentCount = await program.account.splescrowdapp.fetch(splescrowdappKeypair.publicKey)

    expect(currentCount.count).toEqual(42)
  })

  it('Set close the splescrowdapp account', async () => {
    await program.methods
      .close()
      .accounts({
        payer: payer.publicKey,
        splescrowdapp: splescrowdappKeypair.publicKey,
      })
      .rpc()

    // The account should no longer exist, returning null.
    const userAccount = await program.account.splescrowdapp.fetchNullable(splescrowdappKeypair.publicKey)
    expect(userAccount).toBeNull()
  })
})
