'use client'

import { getSplescrowdappProgram, getSplescrowdappProgramId } from '@project/anchor'
import { useConnection } from '@solana/wallet-adapter-react'
import { Cluster, Keypair, PublicKey } from '@solana/web3.js'
import { useMutation, useQuery } from '@tanstack/react-query'
import { useMemo } from 'react'
import toast from 'react-hot-toast'
import { useCluster } from '../cluster/cluster-data-access'
import { useAnchorProvider } from '../solana/solana-provider'
import { useTransactionToast } from '../ui/ui-layout'

export function useSplescrowdappProgram() {
  const { connection } = useConnection()
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const provider = useAnchorProvider()
  const programId = useMemo(() => getSplescrowdappProgramId(cluster.network as Cluster), [cluster])
  const program = useMemo(() => getSplescrowdappProgram(provider, programId), [provider, programId])

  const accounts = useQuery({
    queryKey: ['splescrowdapp', 'all', { cluster }],
    queryFn: () => program.account.splescrowdapp.all(),
  })

  const getProgramAccount = useQuery({
    queryKey: ['get-program-account', { cluster }],
    queryFn: () => connection.getParsedAccountInfo(programId),
  })

  const initialize = useMutation({
    mutationKey: ['splescrowdapp', 'initialize', { cluster }],
    mutationFn: (keypair: Keypair) =>
      program.methods.initialize().accounts({ splescrowdapp: keypair.publicKey }).signers([keypair]).rpc(),
    onSuccess: (signature) => {
      transactionToast(signature)
      return accounts.refetch()
    },
    onError: () => toast.error('Failed to initialize account'),
  })

  return {
    program,
    programId,
    accounts,
    getProgramAccount,
    initialize,
  }
}

export function useSplescrowdappProgramAccount({ account }: { account: PublicKey }) {
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const { program, accounts } = useSplescrowdappProgram()

  const accountQuery = useQuery({
    queryKey: ['splescrowdapp', 'fetch', { cluster, account }],
    queryFn: () => program.account.splescrowdapp.fetch(account),
  })

  const closeMutation = useMutation({
    mutationKey: ['splescrowdapp', 'close', { cluster, account }],
    mutationFn: () => program.methods.close().accounts({ splescrowdapp: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accounts.refetch()
    },
  })

  const decrementMutation = useMutation({
    mutationKey: ['splescrowdapp', 'decrement', { cluster, account }],
    mutationFn: () => program.methods.decrement().accounts({ splescrowdapp: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  const incrementMutation = useMutation({
    mutationKey: ['splescrowdapp', 'increment', { cluster, account }],
    mutationFn: () => program.methods.increment().accounts({ splescrowdapp: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  const setMutation = useMutation({
    mutationKey: ['splescrowdapp', 'set', { cluster, account }],
    mutationFn: (value: number) => program.methods.set(value).accounts({ splescrowdapp: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  return {
    accountQuery,
    closeMutation,
    decrementMutation,
    incrementMutation,
    setMutation,
  }
}
