import React, { useState } from "react"
import axios from "axios"
import dynamic from "next/dynamic"
import { useRouter } from "next/router"

import { money } from "@/assets"
import { checkIfImage } from "@/utils"

import CustomButton from "@/components/CustomButton"
import FormField from "@/components/FormFeild"
import Loader from "@/components/Loader"

// import { useStateContext } from "../context"

const CreateCampaign = () => {
  const router = useRouter()
  const [isLoading, setIsLoading] = useState(false)
  // const { createCampaign } = useStateContext()
  const [form, setForm] = useState({
    title: "",
    description: "",
    image: "",
    story: "",
  })

  const handleFormFieldChange = (
    fieldName: string,
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    setForm({ ...form, [fieldName]: e.target.value })
  }

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()

    checkIfImage(form.image, async (exists) => {
      if (exists) {
        setIsLoading(true)
        console.log(form)
        // await createCampaign({ ...form, target: ethers.utils.parseUnits(form.target, 18) })
        const response = await axios.post("/api/ipfs", form)
        const { url } = response.data
        console.log(url)
        setIsLoading(false)
        // router.push("/")
      } else {
        alert("Provide valid image URL")
        setForm({ ...form, image: "" })
      }
    })
  }

  return (
    <div className="bg-[#1c1c24] flex justify-center items-center flex-col rounded-[10px] sm:p-10 p-4">
      {isLoading && <Loader />}
      <div className="flex justify-center items-center p-[16px] sm:min-w-[380px] bg-[#3a3a43] rounded-[10px]">
        <h1 className="font-epilogue font-bold sm:text-[25px] text-[18px] leading-[38px] text-white">
          Post News Anonymously and Securely
        </h1>
      </div>

      <form onSubmit={handleSubmit} className="w-full mt-[65px] flex flex-col gap-[30px]">
        <div className="flex flex-wrap gap-[40px]">
          <FormField
            labelName="News Title *"
            placeholder="Write a title"
            inputType="text"
            value={form.title}
            handleChange={(e) => handleFormFieldChange("title", e)}
          />
        </div>

        <FormField
          labelName="Description *"
          placeholder="Write your description"
          isTextArea
          value={form.description}
          handleChange={(e) => handleFormFieldChange("description", e)}
        />

        <FormField
          labelName="News image *"
          placeholder="Place image URL of your news"
          inputType="url"
          value={form.image}
          handleChange={(e) => handleFormFieldChange("image", e)}
        />

        <CustomEditor
          // initialData={""}
          onChange={(event, editor) => {
            const data = editor.getData()
            setForm({ ...form, story: data })
          }}
        />

        <div className="w-full flex justify-start items-center p-4 bg-[#8c6dfd] h-[120px] rounded-[10px]">
          <img src={money.src} alt="money" className="w-[40px] h-[40px] object-contain" />
          <h4 className="font-epilogue font-bold text-[25px] text-white ml-[20px]">
            Heads up! Posting fake news may result in the loss of your stake amount!
          </h4>
        </div>

        <div className="flex justify-center items-center mt-[40px]">
          <CustomButton btnType="submit" title="Post your news" styles="bg-[#1dc071]" />
        </div>
      </form>
    </div>
  )
}

export default CreateCampaign

const CustomEditor = dynamic(
  () => {
    return import("@/components/CustomEditor")
  },
  { ssr: false }
)
