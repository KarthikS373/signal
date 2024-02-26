import { Keplr, OfflineAminoSigner, SecretUtils } from "@keplr-wallet/types"

declare global {
  interface Window {
    keplr: Keplr
    leap: Keplr
    getOfflineSignerOnlyAmino: (chainId: string) => OfflineAminoSigner
    getEnigmaUtils: (chainId: string) => SecretUtils
    leap: any
  }
}
