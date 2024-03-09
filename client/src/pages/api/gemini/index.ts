const { GoogleGenerativeAI } = require("@google/generative-ai")

import type { NextApiRequest, NextApiResponse } from "next"

type APITypes<T = unknown | null, E = Error | null> = {
  data: T
  error: E
  message: string
}

const handler = async (req: NextApiRequest, res: NextApiResponse<APITypes>) => {
  try {
    const { form } = req.body
    console.log(form)
    const genAI = new GoogleGenerativeAI(process.env.GEMINI_API_KEY)

    // For text-only input, use the gemini-pro model
    const model = genAI.getGenerativeModel({ model: "gemini-pro" })

    const prompt = `Evaluate the following news article and evaluate whether it is fake or not. The news will be in the form:
    {
      title: string,
      description: string,
      image: string (url),
      story: string (html markup)
    }
    Respond in following JSON format only (always with proper double quotes and wihout newline characters):
    {"fake": boolean (true or false), "confidenceLevel": number (0 to 1), "sources": string[] (list of resources considered to take the decision)}
    
    ${form}
    `

    const result = await model.generateContent(prompt)
    const response = await result.response
    const text = response.text()
    console.log(text)

    res.status(200).send({
      message: "Validation completed!",
      error: null,
      data: {
        text,
      },
    })
  } catch (err: any) {
    res.status(500).send({
      message: "Failed to validate content",
      error: err.message,
      data: null,
    })
  }
}

export default handler
