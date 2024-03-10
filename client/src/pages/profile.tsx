import React, { useEffect, useState } from "react"
import { useRouter } from "next/router"

import { scrt } from "@/assets"
import { useStateContext } from "@/providers/state-context"
import { calculateBarPercentage, daysLeft, slug } from "@/utils"

import CountBox from "@/components/CountBox"
import CustomButton from "@/components/CustomButton"
import FormField from "@/components/FormFeild"
import Loader from "@/components/Loader"

const CampaignDetails = () => {
  const stakeAmount = () => {}
  const unstakeAmount = () => {}
  const withdrawTip = () => {}

  return (
    <div>
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
                  5ff458433270893da736b827486a65b152a1c86725a244e6e47fbf565d7f4bf0
                </h4>
                <p className="mt-[4px] font-epilogue font-normal text-[12px] text-[#808191]">
                  This is not your actual wallet address
                </p>
              </div>
            </div>
            <form onSubmit={() => {}} className="w-full mt-[65px] flex flex-col gap-[30px]">
              <div className="flex flex-wrap gap-[40px]">
                <FormField
                  labelName="Username"
                  placeholder="Username"
                  inputType="text"
                  value={"tsarprince"}
                  // handleChange={(e) => handleFormFieldChange("title", e)}
                />
              </div>
              <div className="flex flex-wrap gap-[40px]">
                <FormField
                  labelName="Profile Photo"
                  inputType="file"
                  // value={"tsarprince"}
                  // handleChange={(e) => handleFormFieldChange("title", e)}
                />
              </div>
              <div className="flex flex-wrap gap-[40px]">
                <FormField
                  labelName="Bio"
                  placeholder="Bio"
                  isTextArea
                  value={`Can fix your computer and explain it in a way that doesn't involve magic words like "cloud" or "the internet fairies."  Bonus: I can also identify all the wires behind your TV (even the mysterious blue one).`}
                  // handleChange={(e) => handleFormFieldChange("title", e)}
                />
              </div>

              <div className="flex justify-center items-center mt-[40px]">
                <CustomButton btnType="submit" title="Edit Your Profile" styles="bg-[#1dc071]" />
              </div>
            </form>
          </div>
        </div>

        <div className="flex-1">
          <CustomButton
            btnType="button"
            title="Withdraw Tip"
            styles="w-full bg-green-400 !text-black ring-0"
            handleClick={withdrawTip}
          />
          {/* <h4 className="font-epilogue font-semibold text-[18px] text-white uppercase">Rate it!</h4> */}

          <div className="mt-[20px] flex flex-col p-4 bg-[#1c1c24] rounded-[10px]">
            <p className="font-epilogue fount-medium text-[20px] leading-[30px] text-center text-[#808191]">
              Handle stake transactions
            </p>
            <div className="mt-[16px]">
              <CustomButton
                btnType="button"
                title="Stake Amount"
                styles="w-full bg-[#8c6dfd] focus:ring-[#8c6dfd71] focus:ring-4"
                handleClick={stakeAmount}
              />
              <CustomButton
                btnType="button"
                title="Unstake Amount"
                styles="w-full bg-[#e64a78] focus:ring-[#e64a7871] focus:ring-4 mt-[16px]"
                handleClick={unstakeAmount}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

export default CampaignDetails
