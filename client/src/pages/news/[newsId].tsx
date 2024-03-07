import React, { useEffect, useState } from "react"
import { useRouter } from "next/router"

import { scrt } from "@/assets"
import { useStateContext } from "@/providers/state-context"
import { calculateBarPercentage, daysLeft, slug } from "@/utils"

import CountBox from "@/components/CountBox"
import CustomButton from "@/components/CustomButton"
import Loader from "@/components/Loader"

const CampaignDetails = () => {
  const { news } = useStateContext()
  const router = useRouter()
  const { newsId } = router.query

  const [isLoading, setIsLoading] = useState(false)
  const [amount, setAmount] = useState("")

  // const [donators, setDonators] = useState([])
  // const { donate, getDonations, contract, address } = useStateContext()
  // const fetchDonators = async () => {
  //   const data = await getDonations(state.pId)

  //   setDonators(data)
  // }

  // useEffect(() => {
  //   if (contract) fetchDonators()
  // }, [contract, address])

  const handleDonate = async () => {
    setIsLoading(true)

    // await donate(state.pId, amount)

    // navigate("/")
    setIsLoading(false)
  }

  const [upvotes, setUpvotes] = useState(53)
  const [downvotes, setDownvotes] = useState(11)

  const state = news?.find((item) => slug(item.title) === newsId)
  if (!state) return null

  return (
    <div>
      {isLoading && <Loader />}

      <div className="w-full flex md:flex-row flex-col mt-10 gap-[30px]">
        <div className="flex-1 flex-col">
          <img
            src={state.image}
            alt="campaign"
            className="w-full h-[410px] object-cover rounded-xl"
          />
          <div className="relative w-full h-[5px] bg-[#3a3a43] mt-2 rounded-full">
            <div
              className="absolute h-full bg-[#4acd8d] rounded-full"
              style={{
                width: `${calculateBarPercentage(upvotes + downvotes, upvotes)}%`,
                maxWidth: "100%",
              }}
            ></div>
          </div>
        </div>

        <div className="flex md:w-[150px] w-full flex-wrap justify-between gap-[30px]">
          <CountBox title="Total ativity" value={"12.5k"} />
          <CountBox title="Upvotes" value={upvotes} />
          <CountBox title="Downvotes" value={downvotes} />
        </div>
      </div>

      <div className="mt-[60px] flex lg:flex-row flex-col gap-5">
        <div className="flex-[2] flex flex-col gap-[40px]">
          <div>
            <h4 className="font-epilogue font-semibold text-[18px] text-white uppercase">
              Creator
            </h4>

            <div className="mt-[20px] flex flex-row items-center flex-wrap gap-[14px]">
              <div className="w-[52px] h-[52px] flex items-center justify-center rounded-full bg-[#2c2f32] cursor-pointer">
                <img src={scrt.src} alt="user" className="w-[60%] h-[60%] object-contain" />
              </div>
              <div>
                <h4 className="font-epilogue font-semibold text-[14px] text-white break-all">
                  ffa209342abfc
                </h4>
                <p className="mt-[4px] font-epilogue font-normal text-[12px] text-[#808191]">
                  {news?.length || 0} Campaigns
                </p>
              </div>
            </div>
          </div>

          <div>
            <h4 className="font-epilogue font-semibold text-[18px] text-white uppercase">
              Description
            </h4>

            <div className="mt-[20px]">
              <p className="font-epilogue font-normal text-[16px] text-[#808191] leading-[26px] text-justify">
                {state.description}
              </p>
            </div>
          </div>

          <div>
            <h4 className="font-epilogue font-semibold text-[18px] text-white uppercase">Story</h4>
            <div
              className="prose max-w-none mt-[20px]"
              dangerouslySetInnerHTML={{ __html: state.story }}
            ></div>
          </div>
        </div>

        <div className="flex-1">
          <h4 className="font-epilogue font-semibold text-[18px] text-white uppercase">Rate it!</h4>

          <div className="mt-[20px] flex flex-col p-4 bg-[#1c1c24] rounded-[10px]">
            <p className="font-epilogue fount-medium text-[20px] leading-[30px] text-center text-[#808191]">
              Found the article legit?
            </p>
            <div className="mt-[30px]">
              <input
                type="number"
                placeholder="ETH 0.1"
                step="0.01"
                className="w-full py-[10px] sm:px-[20px] px-[15px] outline-none border-[1px] border-[#3a3a43] bg-transparent font-epilogue text-white text-[18px] leading-[30px] placeholder:text-[#4b5264] rounded-[10px]"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
              />

              <div className="my-[20px] p-4 bg-[#13131a] rounded-[10px]">
                <h4 className="font-epilogue font-semibold text-[14px] leading-[22px] text-white">
                  Back it because you believe in it.
                </h4>
                <p className="mt-[20px] font-epilogue font-normal leading-[22px] text-[#808191]">
                  Support the news for no reward, just because it speaks to you.
                </p>
              </div>

              <CustomButton
                btnType="button"
                title="Upvote 👍"
                styles="w-full bg-[#8c6dfd]"
                handleClick={() => setUpvotes(upvotes + 1)}
              />
              <CustomButton
                btnType="button"
                title="Downvote 👎"
                styles="w-full bg-[#e64a78] mt-[10px]"
                handleClick={() => setDownvotes(downvotes + 1)}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

export default CampaignDetails
