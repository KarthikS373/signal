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

    const prompt = `Evaluate the following news and evaluate whether it is fake or not.
    Respond in following JSON format only (always with proper double quotes and wihout newline characters):
    {"fake": boolean (true or false), "confidenceLevel": number (0 to 1), "sources": string[] (list of resources considered to take the decision)}
    
    ${form.title}
    `

    const result = await model.generateContent(prompt)
    const response = await result.response
    console.log(JSON.stringify(response, null, 2))

    if (response.promptFeedback?.blockReason === "SAFETY") {
      // if (Array.isArray(response.promptFeedback?.safetyRatings)) {
      return res.status(200).send({
        message: "Skam detected!",
        error: null,
        data: { fake: true, confidenceLevel: 1, sources: response.promptFeedback },
      })
    }
    const text = JSON.parse(response.text())
    console.log(text)

    return res.status(200).send({
      message: "Validation completed!",
      error: null,
      data: {
        ...text,
      },
    })
  } catch (err) {
    return res.status(500).send({
      message: "Failed to validate content",
      error: new Error(err instanceof Error ? err.message : "Something went wrong"),
      data: null,
    })
  }
}

export default handler
