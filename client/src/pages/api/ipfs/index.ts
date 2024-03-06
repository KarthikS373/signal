import type { NextApiRequest, NextApiResponse } from "next"

import { ThirdwebStorage } from "@thirdweb-dev/storage"

type APITypes<T = unknown | null, E = Error | null> = {
  data: T
  error: E
  message: string
}

const handler = async (req: NextApiRequest, res: NextApiResponse<APITypes>) => {
  try {
    const body = req.body
    const storage = new ThirdwebStorage({
      secretKey: process.env.THIRDWEB_SECRET_KEY,
    })
    const uri = await storage.upload(body)
    const url = await storage.resolveScheme(uri)
    console.log({ uri, url })

    // // You can also download the data from the uri
    // const data = await storage.downloadJSON(uri)
    res.status(200).send({
      message: "Uploaded successfully!",
      error: null,
      data: {
        uri,
        url,
      },
    })
  } catch (err: any) {
    res.status(500).send({
      message: "Failed to upload",
      error: err.message,
      data: null,
    })
  }
}

export default handler
