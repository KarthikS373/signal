import React from "react"

import { UserProvider } from "./user-context"
import { AuthProvider } from "./auth-context"

interface ProvidersProps {
  children: React.ReactNode
}

const Providers: React.FC<ProvidersProps> = ({ children }) => {
  return (
    <UserProvider>
      <AuthProvider>
        <>{children}</>
      </AuthProvider>
    </UserProvider>
  )
}

export default Providers
