import React, { useState } from "react"
import axios from "axios"
import dynamic from "next/dynamic"
import { useRouter } from "next/router"

import { money } from "@/assets"
import { useStateContext } from "@/providers/state-context"
import { checkIfImage } from "@/utils"

import MyDialog from "@/components/ConfirmationDialog"
import CustomButton from "@/components/CustomButton"
import FormField from "@/components/FormFeild"
import Loader from "@/components/Loader"

export type News = {
  title: string
  description: string
  image: string
  story: string
}

const CreateCampaign = () => {
  const router = useRouter()
  let [isOpen, setIsOpen] = useState(true)

  const [isLoading, setIsLoading] = useState(false)
  const { postNews } = useStateContext()
  const [form, setForm] = useState<News>({
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

        try {
          const geminiResponse = await axios.post("/api/gemini", { form: form })
          console.log(geminiResponse.data)
          const { fake, confidenceLevel, sources } = geminiResponse.data.data
          if (fake && confidenceLevel > 0.8) {
            alert(
              "The news you are trying to post is fake OR it breaches our SAFETY STANDARDS. Pls report the issue if you are having trouble."
            )
            return
          }
          const response = await axios.post("/api/ipfs", form)
          const { uri, url } = response.data.data
          console.log({ uri, url })
          const tx = await postNews(url)
          console.log("Tx result: ", tx)
          // setForm({ title: "", description: "", image: "", story: "" })
          router.push("/")
        } catch (error) {
          alert(
            "No creator profile associated with your account. Pls create one with your stake amount before continuing."
          )
          console.log(error)
        } finally {
          setIsLoading(false)
        }
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

        <div className="w-full flex justify-start items-center p-4 bg-[#8c6dfd] rounded-[10px]">
          <img src={money.src} alt="money" className="w-[40px] h-[40px] object-contain" />
          <h4 className="font-epilogue font-bold md:text-[25px] text-white ml-[20px]">
            Heads up! Posting fake news may result in the loss of your stake amount!
          </h4>
        </div>

        <label>
          <input type="checkbox" required />
          <span className="ml-4 font-epilogue font-medium text-[16px] leading-[22px] text-[#808191]">
            This news is valid and genuine to the best of my knowledge.
          </span>
        </label>

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
