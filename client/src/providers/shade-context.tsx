import React, { createContext, useCallback, useContext, useState } from "react"

import { News } from "@/pages/post-news"
import shades from "@shadeprotocol/shadejs"

interface IContext {
  fetchSCRTPrice: () => void
}

const ShadeContext = createContext<IContext>({} as IContext)

const contract_address =
  process.env.CONTRACT_ADDRESS ?? "secret190a5htfnmm5a4a5wnznj56dk2ukvm5tc90shg8"
const code_hash =
  process.env.CODE_HASH ?? "21065c3c46e332b8f0530a30f8374a8674e585a268ff0004e34c443ebd456c2f"

export const ShadeProvider = ({ children }: { children: React.ReactNode }) => {
  const fetchSCRTPrice = useCallback(() => {
    if (!window || typeof window === "undefined") return

    // const op = shades.queryPrice({
    //   contractAddress: "secret1k0jntykt7e4g3y88ltc60czgjuqdy4c9e8fzek",
    //   oracleKey: "SCRT",
    // })
    // console.log("SCRT price feeds:", op)
  }, [])

  return <ShadeContext.Provider value={{ fetchSCRTPrice }}>{children}</ShadeContext.Provider>
}

export const useShadeContext = () => {
  return useContext(ShadeContext)
}
