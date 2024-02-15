import React from "react"
import DisplayCampaigns from "@/components/DisplayCampaigns"

interface HomePageProps {
  path?: string
}

const HomePage: React.FC<HomePageProps> = () => {
  return (
    <div>
      <DisplayCampaigns title="All Campaigns" isLoading={false} />
    </div>
  )
}

export default HomePage
