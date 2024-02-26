import React, { useState, useEffect, useContext, createContext } from "react"
import { SecretNetworkClient } from "secretjs"

export type UserData = {
  leap?: {
    walletName: string
    walletAddress: string
  }
  keplr?: {
    walletName: string
    walletAddress: string
    walletClient: SecretNetworkClient
  }
}

const AuthContext = createContext<{
  loading: boolean
  setLoading: React.Dispatch<React.SetStateAction<boolean>>
  userData: UserData
  setUserData: React.Dispatch<React.SetStateAction<UserData>>
}>({
  loading: false,
  setLoading: () => {},
  userData: {},
  setUserData: () => {},
})

const AuthProvider = ({ children }: { children: React.ReactNode }) => {
  const [loading, setLoading] = useState(false)
  const [userData, setUserData] = useState<UserData>({})

  return (
    <AuthContext.Provider value={{ loading, setLoading, userData, setUserData }}>
      {children}
    </AuthContext.Provider>
  )
}

const useAuth = () => {
  const c = useContext(AuthContext)

  if (c === undefined) {
    throw new Error("useAuth must be used within a AuthProvider")
  }

  return c
}

export { AuthProvider, useAuth }
