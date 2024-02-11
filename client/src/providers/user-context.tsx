import React, { useState } from "react"

interface IUserContext {
  user: IUser | null
}

interface IUserProvider {
  children: React.ReactNode
}

const Context = React.createContext<IUserContext>({} as IUserContext)

const UserProvider = ({ children }: IUserProvider) => {
  const [user] = useState<IUser | null>(null)

  return <Context.Provider value={{ user }}>{children}</Context.Provider>
}

const useUserContext = () => {
  const c = React.useContext(Context)

  if (c === undefined) {
    throw new Error("useUserContext must be used within a UserProvider")
  }

  return c
}

export { UserProvider, useUserContext }
