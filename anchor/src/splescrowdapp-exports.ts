// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor'
import { Cluster, PublicKey } from '@solana/web3.js'
import SplescrowdappIDL from '../target/idl/splescrowdapp.json'
import type { Splescrowdapp } from '../target/types/splescrowdapp'

// Re-export the generated IDL and type
export { Splescrowdapp, SplescrowdappIDL }

// The programId is imported from the program IDL.
export const SPLESCROWDAPP_PROGRAM_ID = new PublicKey(SplescrowdappIDL.address)

// This is a helper function to get the Splescrowdapp Anchor program.
export function getSplescrowdappProgram(provider: AnchorProvider, address?: PublicKey) {
  return new Program({ ...SplescrowdappIDL, address: address ? address.toBase58() : SplescrowdappIDL.address } as Splescrowdapp, provider)
}

// This is a helper function to get the program ID for the Splescrowdapp program depending on the cluster.
export function getSplescrowdappProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
      // This is the program ID for the Splescrowdapp program on devnet and testnet.
      return new PublicKey('coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF')
    case 'mainnet-beta':
    default:
      return SPLESCROWDAPP_PROGRAM_ID
  }
}
