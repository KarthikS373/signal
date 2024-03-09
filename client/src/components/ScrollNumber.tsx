import React, { useCallback, useRef } from "react"
import clsx from "clsx"

const ScrollNumber = ({ letter }: { letter: string }) => {
  const containerRef = useCallback(
    (container: HTMLDivElement) => {
      if (container) {
        const height = digitRef.current?.getBoundingClientRect().height!
        container.scrollTo(0, height * Number(letter))
      }
    },
    [letter]
  )
  const digitRef = useRef<HTMLSpanElement>(null)

  return (
    <div
      className="h-[1.5em] overflow-hidden pt-3 -mx-3 scroll-smooth"
      style={{
        textOrientation: "upright",
        writingMode: "vertical-rl",
      }}
      ref={containerRef}
    >
      {Array.from({ length: 10 }).map((_, i) => (
        <span
          key={i}
          className={clsx(
            letter == i.toString() ? "opacity-100" : "opacity-0",
            "transition duration-300"
          )}
          ref={digitRef}
        >
          {i}
        </span>
      ))}
    </div>
  )
}

export default ScrollNumber
