import React from "react"

import { UserProvider } from "./user-context"

interface ProvidersProps {
  children: React.ReactNode
}

const Providers: React.FC<ProvidersProps> = ({ children }) => {
  return (
    <UserProvider>
      <>{children}</>
    </UserProvider>
  )
}

export default Providers
