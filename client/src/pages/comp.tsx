import React, { useEffect } from "react"

import { useStateContext } from "@/providers/state-context"

const Component = () => {
  const { lockDeposit, unlockDeposit, tipCreator, creatorWithdrawTip } = useStateContext()

  const lock = () => {
    try {
      lockDeposit("1")
    } catch {
      // Show toast
    }
  }

  const unlock = () => {
    try {
      unlockDeposit("1")
    } catch {
      // Show toast
    }
  }

  const tip = () => {
    try {
      tipCreator("1", "creator")
    } catch {
      // Show toast
    }
  }

  useEffect(() => {}, [])

  return (
    <div className="flex flex-col gap-2">
      <button onClick={() => lock()} className="bg-red-100 px-4 py-2 text-black rounded-lg">
        Creator lock funds
      </button>
      <button onClick={() => unlock()} className="bg-red-100 px-4 py-2 text-black rounded-lg">
        Creator unlock funds
      </button>
    </div>
  )
}

export default Component
