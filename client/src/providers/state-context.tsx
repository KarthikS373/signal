import React, { createContext, useContext, useState } from "react"
import { TxResponse } from "secretjs"

import { News } from "@/pages/post-news"

import { useAuth } from "./auth-context"

const StateContext = createContext<{
  getAllNews: () => Promise<any>
  postNews: (ipfsUrl: string) => Promise<any>
  news: News[] | null
  setNews: React.Dispatch<React.SetStateAction<News[] | null>>
  createCreatorProfile: () => void
  lockDeposit: (amount: string) => Promise<TxResponse>
  unlockDeposit: (amount: string) => Promise<TxResponse>
  tipCreator: (amount: string, creator: string) => Promise<TxResponse>
  creatorWithdrawTip: (amount: string) => Promise<TxResponse>
}>({
  getAllNews: async () => {},
  postNews: async (ipfsUrl: string) => {},
  news: null,
  setNews: () => {},
  createCreatorProfile: () => {},
  lockDeposit: async (amount: string) => ({}) as TxResponse,
  unlockDeposit: async (amount: string) => ({}) as TxResponse,
  tipCreator: async (amount: string, creator: string) => ({}) as TxResponse,
  creatorWithdrawTip: async (amount: string) => ({}) as TxResponse,
})

const contract_address =
  process.env.NEXT_PUBLIC_CONTRACT_ADDRESS ?? "secret190a5htfnmm5a4a5wnznj56dk2ukvm5tc90shg8"
const code_hash =
  process.env.NEXT_PUBLIC_CODE_HASH ??
  "21065c3c46e332b8f0530a30f8374a8674e585a268ff0004e34c443ebd456c2f"

export const StateProvider = ({ children }: { children: React.ReactNode }) => {
  const [news, setNews] = useState<News[] | null>(null)
  const { userData } = useAuth()

  const client = userData.keplr?.walletClient
  const sender = userData.keplr?.walletAddress

  if (!client || !sender) {
    return <>{children}</>
  }

  // const fetchRecentTransactions = async () => {}

  const createCreatorProfile = () => {
    client.query.compute.queryContract({
      contract_address,
      code_hash,
      query: { create_creator_profile: {} },
    })
  }

  const lockDeposit = (amount: string) => {
    return client.tx.compute.executeContract(
      {
        sender,
        contract_address,
        code_hash,
        msg: {
          lock_funds: {
            amount,
          },
        },
        sent_funds: [
          {
            amount: amount,
            denom: "uscrt",
          },
        ],
      },
      {
        gasLimit: 100_000,
      }
    )
  }

  const unlockDeposit = (amount: string) => {
    return client.tx.compute.executeContract(
      {
        sender,
        contract_address,
        code_hash,
        msg: {
          unlock_funds: {
            amount,
          },
        },
        sent_funds: [],
      },
      {
        gasLimit: 100_000,
      }
    )
  }

  const tipCreator = (amount: string, creator: string) => {
    return client.tx.compute.executeContract(
      {
        sender,
        contract_address,
        code_hash,
        msg: {
          tip_creator: {
            creator,
          },
        },
        sent_funds: [
          {
            amount,
            denom: "uscrt",
          },
        ],
      },
      {
        gasLimit: 100_000,
      }
    )
  }

  const creatorWithdrawTip = (amount: string) => {
    return client.tx.compute.executeContract(
      {
        sender,
        contract_address,
        code_hash,
        msg: {
          withdraw_tip: {
            amount,
          },
        },
        sent_funds: [],
      },
      {
        gasLimit: 100_000,
      }
    )
  }

  const getAllNews = () =>
    client.query.compute.queryContract({
      contract_address,
      code_hash, // optional but way faster
      query: { get_all_news_items: {} },
    })

  const postNews = (ipfsUrl: string) =>
    client.tx.compute.executeContract(
      {
        sender,
        contract_address,
        code_hash,
        msg: {
          post_news: {
            content: ipfsUrl,
          },
        },
        sent_funds: [],
      },
      {
        gasLimit: 100_000,
      }
    )

  return (
    <StateContext.Provider
      value={{
        createCreatorProfile,
        lockDeposit,
        unlockDeposit,
        getAllNews,
        postNews,
        news,
        setNews,
        tipCreator,
        creatorWithdrawTip,
      }}
    >
      {children}
    </StateContext.Provider>
  )
}

export const useStateContext = () => {
  return useContext(StateContext)
}
