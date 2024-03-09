import React from "react"

import { AuthProvider } from "./auth-context"
import { StateProvider } from "./state-context"

interface ProvidersProps {
  children: React.ReactNode
}

const Providers: React.FC<ProvidersProps> = ({ children }) => {
  return (
    <AuthProvider>
      <StateProvider>
        <>{children}</>
      </StateProvider>
    </AuthProvider>
  )
}

export default Providers
