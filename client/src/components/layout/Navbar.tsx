import React, { useState } from "react"
import Image from "next/image"
import Link from "next/link"
import { useRouter } from "next/router"

import { logo, menu, search, thirdweb } from "@/assets"
import { navlinks } from "@/data"
import { SecretNetworkClient } from "secretjs"

import CustomButton from "@/components/CustomButton"
import { useAuth } from "@/providers/auth-context"

const Navbar = () => {
  const router = useRouter()
  const [isActive, setIsActive] = useState("dashboard")
  const [toggleDrawer, setToggleDrawer] = useState(false)

  const { loading, setLoading, userData, setUserData } = useAuth()

  const CHAIN_ID = {
    leap: "juno-1",
    keplr: "pulsar-3",
  }
  const LCD = {
    keplr: "https://api.pulsar3.scrttestnet.com",
  }

  const conectLeap = async () => {
    try {
      setLoading(true)
      if (typeof window !== "undefined") {
        // leap
        const leapProvider = window.leap
        if (!leapProvider) {
          alert("Pls install Leap wallet. Thx!")
        } else {
          const key = await leapProvider.getKey(CHAIN_ID["leap"])
          const { name, bech32Address } = key
          setUserData((userData) => ({
            ...userData,
            leap: { walletName: name, walletAddress: bech32Address },
          }))
        }
      }
    } catch (err) {
      console.log(err)
    } finally {
      setLoading(false)
    }
  }

  const connectKeplr = async () => {
    try {
      setLoading(true)
      if (typeof window !== "undefined") {
        // keplr
        const keplr = window.keplr
        if (!keplr) {
          alert("Pls install Keplr wallet. Thx!")
        } else {
          // Enabling before using the Keplr is recommended.
          // This method will ask the user whether or not to allow access if they haven't visited this website.
          // Also, it will request user to unlock the wallet if the wallet is locked.
          await keplr.enable(CHAIN_ID["keplr"])
          const offlineSigner = window.getOfflineSignerOnlyAmino(CHAIN_ID["keplr"])

          // You can get the address/public keys by `getAccounts` method.
          // It can return the array of address/public key.
          // But, currently, Keplr extension manages only one address/public key pair.
          // XXX: This line is needed to set the sender address for SigningCosmosClient.
          const accounts = await offlineSigner.getAccounts()
          const secretjs = new SecretNetworkClient({
            url: LCD["keplr"],
            chainId: CHAIN_ID["keplr"],
            wallet: offlineSigner,
            walletAddress: accounts[0].address,
            encryptionUtils: window.getEnigmaUtils(CHAIN_ID["keplr"]),
          })
          setUserData((userData) => ({
            ...userData,
            keplr: {
              walletName: "SCRT",
              walletAddress: accounts[0].address,
              walletClient: secretjs,
            },
          }))
        }
      }
    } catch (err) {
      console.log(err)
    } finally {
      setLoading(false)
    }
  }

  console.log({ loading, userData })

  const address = userData.keplr?.walletAddress

  return (
    <div className="flex md:flex-row flex-col-reverse justify-between mb-[35px] gap-6">
      <div className="lg:flex-1 flex flex-row max-w-[458px] py-2 pl-4 pr-2 h-[52px] bg-[#1c1c24] rounded-[100px]">
        <input
          type="text"
          placeholder="Search for campaigns"
          className="flex w-full font-epilogue font-normal text-[14px] placeholder:text-[#4b5264] text-white bg-transparent outline-none"
        />

        <div className="w-[72px] h-full rounded-[20px] bg-[#4acd8d] flex justify-center items-center cursor-pointer">
          <Image src={search} alt="search" className="w-[15px] h-[15px] object-contain" />
        </div>
      </div>

      <div className="sm:flex hidden flex-row justify-end gap-4">
        <CustomButton
          btnType="button"
          title={address ? "Create a campaign" : "Connect"}
          styles={address ? "bg-[#1dc071]" : "bg-[#8c6dfd]"}
          handleClick={() => {
            if (address) router.push("/create-campaign")
            else connectKeplr()
          }}
        />

        <Link href="/profile">
          <div className="w-[52px] h-[52px] rounded-full bg-[#2c2f32] flex justify-center items-center cursor-pointer">
            <Image src={thirdweb} alt="user" className="w-[60%] h-[60%] object-contain" />
          </div>
        </Link>
      </div>

      {/* Small screen navigation */}
      <div className="sm:hidden flex justify-between items-center relative">
        <div className="w-[40px] h-[40px] rounded-[10px] bg-[#2c2f32] flex justify-center items-center cursor-pointer">
          <Image src={logo} alt="user" className="w-[60%] h-[60%] object-contain" />
        </div>

        <Image
          src={menu}
          alt="menu"
          className="w-[34px] h-[34px] object-contain cursor-pointer"
          onClick={() => setToggleDrawer((prev) => !prev)}
        />

        <div
          className={`absolute top-[60px] right-0 left-0 bg-[#1c1c24] z-10 shadow-secondary py-4 ${
            !toggleDrawer ? "-translate-y-[100vh]" : "translate-y-0"
          } transition-all duration-700`}
        >
          <ul className="mb-4">
            {navlinks.map((link) => (
              <li
                key={link.name}
                className={`flex p-4 ${isActive === link.name && "bg-[#3a3a43]"}`}
                onClick={() => {
                  setIsActive(link.name)
                  setToggleDrawer(false)
                  router.push(link.link)
                }}
              >
                <Image
                  src={link.imgUrl}
                  alt={link.name}
                  className={`w-[24px] h-[24px] object-contain ${
                    isActive === link.name ? "grayscale-0" : "grayscale"
                  }`}
                />
                <p
                  className={`ml-[20px] font-epilogue font-semibold text-[14px] ${
                    isActive === link.name ? "text-[#1dc071]" : "text-[#808191]"
                  }`}
                >
                  {link.name}
                </p>
              </li>
            ))}
          </ul>

          <div className="flex mx-4">
            <CustomButton
              btnType="button"
              title={address ? "Create a campaign" : "Connect"}
              styles={address ? "bg-[#1dc071]" : "bg-[#8c6dfd]"}
              handleClick={() => {
                if (address) router.push("create-campaign")
                else connectKeplr()
              }}
            />
          </div>
        </div>
      </div>
    </div>
  )
}

export default Navbar
