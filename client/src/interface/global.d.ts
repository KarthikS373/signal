import { Keplr, OfflineAminoSigner, SecretUtils } from "@keplr-wallet/types"

declare type Callback<T = unknown, P = Error | null> = (error?: P, data?: T) => void

declare global {
  interface Window {
    keplr: Keplr
    leap: Keplr
    getOfflineSignerOnlyAmino: (chainId: string) => OfflineAminoSigner
    getEnigmaUtils: (chainId: string) => SecretUtils
  }
}
