import type { NextApiRequest, NextApiResponse } from "next"

type APITypes<T = unknown | null, E = Error | null> = {
  data: T
  error: E
  message: string
}

const handler = (req: NextApiRequest, res: NextApiResponse<APITypes>) => {
  res.status(200).send({
    message: "Connection Successful",
    error: null,
    data: null,
  })
}

export default handler
