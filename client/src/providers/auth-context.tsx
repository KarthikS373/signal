import React, { createContext, useContext, useState } from "react"
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

const CHAIN_ID = {
  leap: "juno-1",
  keplr: "pulsar-3",
}
const LCD = {
  keplr: "https://api.pulsar3.scrttestnet.com",
}

const AuthContext = createContext<{
  loading: boolean
  userData: UserData
  connectKeplr: () => void
  connectLeap: () => void
}>({
  loading: false,
  userData: {},
  connectKeplr: () => {},
  connectLeap: () => {},
})

const AuthProvider = ({ children }: { children: React.ReactNode }) => {
  const [loading, setLoading] = useState(false)
  const [userData, setUserData] = useState<UserData>({})

  const connectLeap = async () => {
    try {
      setLoading(true)
      if (typeof window !== "undefined") {
        // leap
        const leapProvider = window.leap
        if (!leapProvider) {
          alert("Pls install Leap wallet. Thx!")
        } else {
          const key = await leapProvider.getKey(CHAIN_ID["leap"])
          const { name, bech32Address } = key
          setUserData((userData) => ({
            ...userData,
            leap: { walletName: name, walletAddress: bech32Address },
          }))
        }
      }
    } catch (err) {
      console.log(err)
    } finally {
      setLoading(false)
    }
  }

  const connectKeplr = async () => {
    try {
      setLoading(true)
      if (typeof window !== "undefined") {
        // keplr
        const keplr = window.keplr
        if (!keplr) {
          alert("Pls install Keplr wallet. Thx!")
        } else {
          // Enabling before using the Keplr is recommended.
          // This method will ask the user whether or not to allow access if they haven't visited this website.
          // Also, it will request user to unlock the wallet if the wallet is locked.
          await keplr.enable(CHAIN_ID["keplr"])
          const offlineSigner = window.getOfflineSignerOnlyAmino(CHAIN_ID["keplr"])

          // You can get the address/public keys by `getAccounts` method.
          // It can return the array of address/public key.
          // But, currently, Keplr extension manages only one address/public key pair.
          // XXX: This line is needed to set the sender address for SigningCosmosClient.
          const accounts = await offlineSigner.getAccounts()
          const secretjs = new SecretNetworkClient({
            url: LCD["keplr"],
            chainId: CHAIN_ID["keplr"],
            wallet: offlineSigner,
            walletAddress: accounts[0].address,
            encryptionUtils: window.getEnigmaUtils(CHAIN_ID["keplr"]),
          })
          setUserData((userData) => ({
            ...userData,
            keplr: {
              walletName: "SCRT",
              walletAddress: accounts[0].address,
              walletClient: secretjs,
            },
          }))
        }
      }
    } catch (err) {
      console.log(err)
    } finally {
      setLoading(false)
    }
  }

  return (
    <AuthContext.Provider value={{ loading, userData, connectKeplr, connectLeap }}>
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
