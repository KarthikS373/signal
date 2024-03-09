export const daysLeft = (deadline: number) => {
  const difference = new Date(deadline).getTime() - Date.now()
  const remainingDays = difference / (1000 * 3600 * 24)

  return remainingDays.toFixed(0)
}

export const calculateBarPercentage = (goal: number, raisedAmount: number) => {
  const percentage = (raisedAmount * 100) / goal

  return percentage
}

export const checkIfImage = (url: string, callback: (result: boolean) => void) => {
  const img = new Image()
  img.src = url

  if (img.complete) callback(true)

  img.onerror = () => callback(false)
}

export const slug = (text: string) => {
  return text
    .toLowerCase()
    .replace(/ /g, "-")
    .replace(/[^\w-]+/g, "")
}
