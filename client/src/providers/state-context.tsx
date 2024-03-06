import React, { createContext, useContext, useState } from "react"
import { TxResponse } from "secretjs"

import { News } from "@/pages/post-news"

import { useAuth } from "./auth-context"

const StateContext = createContext<{
  getAllNews: () => Promise<any>
  postNews: (ipfsUrl: string) => Promise<any>
  news: News[] | null
  setNews: React.Dispatch<React.SetStateAction<News[] | null>>
}>({
  getAllNews: async () => {},
  postNews: async (ipfsUrl: string) => {},
  news: null,
  setNews: () => {},
})

const contract_address = "secret190a5htfnmm5a4a5wnznj56dk2ukvm5tc90shg8"
const code_hash = "21065c3c46e332b8f0530a30f8374a8674e585a268ff0004e34c443ebd456c2f"

export const StateProvider = ({ children }: { children: React.ReactNode }) => {
  const [news, setNews] = useState<News[] | null>(null)
  const { userData } = useAuth()

  // TODO: Fix client not available
  const client = userData.keplr?.walletClient!
  const sender = userData.keplr?.walletAddress!

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
        getAllNews,
        postNews,
        news,
        setNews,
      }}
    >
      {children}
    </StateContext.Provider>
  )
}

export const useStateContext = () => {
  return useContext(StateContext)
}
