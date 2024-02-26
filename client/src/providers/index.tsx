import React from "react"

import { AuthProvider } from "./auth-context"

interface ProvidersProps {
  children: React.ReactNode
}

const Providers: React.FC<ProvidersProps> = ({ children }) => {
  return (
    <AuthProvider>
      <>{children}</>
    </AuthProvider>
  )
}

export default Providers
