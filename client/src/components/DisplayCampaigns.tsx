import React, { useEffect, useState } from "react"
import axios from "axios"
import Image from "next/image"
import { useRouter } from "next/router"
import { v4 as uuidv4 } from "uuid"

import { loader } from "@/assets"
import { News } from "@/pages/post-news"
import { useAuth } from "@/providers/auth-context"
import { useStateContext } from "@/providers/state-context"
import { slug } from "@/utils"

import FundCard from "@/components/FundCard"

export interface Campaign {
  owner: string
  title: string
  description: string
  target: string
  deadline: number
  amountCollected: string
  image: string
  pId: number
}

export type getAllNewsResponse = {
  creator: string
  content: string
}[]

const Displaynews = ({ title }: { title: string }) => {
  const router = useRouter()
  const [isLoading, setIsLoading] = useState(false)

  const { getAllNews, news, setNews } = useStateContext()
  const { userData } = useAuth()
  useEffect(() => {
    const fetchData = async () => {
      if (userData.keplr?.walletClient) {
        setIsLoading(true)

        try {
          const newsHashes = (await getAllNews()) as getAllNewsResponse
          const newsResults = await Promise.all(
            newsHashes
              .filter((data) => data.content.startsWith("https://"))
              .map((data) => axios.get(data.content))
          )
          console.log(newsHashes)

          setNews(newsResults.map((result) => result.data))
        } catch (err) {
          console.log(err)
        } finally {
          setIsLoading(false)
        }
      }
    }
    fetchData()
  }, [userData])

  const handleNavigate = (campaign: News) => {
    router.push(`/news/${slug(campaign.title)}`)
  }

  if (!userData.keplr?.walletAddress) {
    return (
      <h1 className="font-epilogue font-semibold text-[18px] text-white text-left">
        {"Pls connect your wallet >:("}
      </h1>
    )
  }

  return (
    <div>
      <h1 className="font-epilogue font-semibold text-[18px] text-white text-left">
        {title} ({news?.length || 0})
      </h1>

      <div className="flex flex-wrap mt-[20px] gap-[26px]">
        {isLoading && (
          <Image src={loader} alt="loader" className="w-[100px] h-[100px] object-contain" />
        )}

        {!isLoading && news?.length === 0 && (
          <p className="font-epilogue font-semibold text-[14px] leading-[30px] text-[#818183]">
            You have not posted any news yet
          </p>
        )}

        {!isLoading &&
          news?.length &&
          news?.map((campaign) => (
            <FundCard
              key={uuidv4()}
              campaign={campaign}
              handleClick={() => handleNavigate(campaign)}
            />
          ))}
      </div>
    </div>
  )
}

export default Displaynews
