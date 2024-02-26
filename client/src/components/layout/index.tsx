import React from "react"

interface LayoutProps {
  children: React.ReactNode
}
import Navbar from "@/components/layout/Navbar"
import Sidebar from "@/components/layout/Sidebar"

const Layout: React.FC<LayoutProps> = ({ children }) => {
  return (
    <div className="relative sm:-8 p-4 bg-[#13131a] text-[#fff] min-h-screen flex flex-row">
      <div className="sm:flex hidden mr-10 relative">
        <Sidebar />
      </div>

      <div className="flex-1 max-sm:w-full max-w-[1280px] mx-auto sm:pr-5">
        <Navbar />

        {children}
      </div>
    </div>
  )
}

export default Layout
