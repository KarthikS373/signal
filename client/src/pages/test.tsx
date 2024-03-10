import React, { useEffect } from "react"
import dynamic from "next/dynamic"

const Component = dynamic(() => import("./comp"), { ssr: false })

const TestPage = () => {
  return <Component />
}

export default TestPage
