import React from "react"
import Image from "next/image"
import { useRouter } from "next/router"
import { v4 as uuidv4 } from "uuid"

import { loader } from "@/assets"

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

const DisplayCampaigns = ({ title, isLoading }: { title: string; isLoading: boolean }) => {
  const router = useRouter()

  const handleNavigate = (campaign: Campaign) => {
    router.push(`/campaign-details/${campaign.title}`)
  }

  const campaigns = [
    {
      owner: "0x25FCbC1e39Ca9b351FE907fC6F0E1788517E5890",
      title: "scem ho gya",
      description: "posa gya",
      target: "0.000000000000000001",
      deadline: 1707775714036,
      amountCollected: "0.0",
      image: "https://images.unsplash.com/photo-1706554596177-35b0a05a082e",
      pId: 0,
    },
    {
      owner: "0x25FCbC1e39Ca9b351FE907fC6F0E1788517E5890",
      title: "crazy dave",
      description: "need posa asap gib",
      target: "0.5",
      deadline: 1709164800000,
      amountCollected: "0.01",
      image: "https://images.unsplash.com/photo-1496449903678-68ddcb189a24",
      pId: 1,
    },
  ]
  return (
    <div>
      <h1 className="font-epilogue font-semibold text-[18px] text-white text-left">
        {title} ({campaigns.length})
      </h1>

      <div className="flex flex-wrap mt-[20px] gap-[26px]">
        {isLoading && (
          <Image src={loader} alt="loader" className="w-[100px] h-[100px] object-contain" />
        )}

        {!isLoading && campaigns.length === 0 && (
          <p className="font-epilogue font-semibold text-[14px] leading-[30px] text-[#818183]">
            You have not created any campigns yet
          </p>
        )}

        {!isLoading &&
          campaigns.length > 0 &&
          campaigns.map((campaign) => (
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

export default DisplayCampaigns
