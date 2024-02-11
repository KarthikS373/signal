import Head from "next/head"
import type { FC } from "react"

import metadata from "@/data/metadata.json"

interface MetaTagsProps {
  description?: string
  title?: string
  page?: string
}

const {
  default_og: DEFAULT_OG,
  description: DESCRIPTION,
  title: APP_NAME,
  twitter: TWITTER,
  website: WEBSITE,
} = metadata

const MetaTags: FC<MetaTagsProps> = ({ description = DESCRIPTION, title = APP_NAME, page }) => {
  return (
    <Head>
      <title key="title">{`${title} ${page ? `| ${page}` : ""}`}</title>
      <meta content={description} name="description" />
      <meta
        content="width=device-width, initial-scale=1, maximum-scale=5, viewport-fit=cover"
        name="viewport"
      />
      <link href={WEBSITE} rel="canonical" />

      <meta content={WEBSITE} property="og:url" />
      <meta content={APP_NAME} property="og:site_name" />
      <meta content={title} property="og:title" />
      <meta content={description} property="og:description" />
      <meta content={DEFAULT_OG} property="og:image" />

      <meta content="summary_large_image" property="twitter:card" />
      <meta content={APP_NAME} property="twitter:site" />
      <meta content={title} property="twitter:title" />
      <meta content={description} property="twitter:description" />
      <meta content={DEFAULT_OG} property="twitter:image" />
      <meta content={TWITTER.size.width} property="twitter:image:width" />
      <meta content={TWITTER.size.height} property="twitter:image:height" />
      <meta content={TWITTER.creator} property="twitter:creator" />

      <link
        href="/opensearch.xml"
        rel="search"
        title={APP_NAME}
        type="application/opensearchdescription+xml"
      />

      {/* Prefetch and Preconnect */}
      {/* <link href="" rel="preconnect" /> */}
      {/* <link href="" rel="dns-prefetch" /> */}

      {/* Icons */}
      {/* <link href="" rel="apple-touch-icon" sizes="180x180" /> */}
      {/* <link href="" rel="icon" sizes="32x32" type="image/png" /> */}
      {/* <link href="" rel="icon" sizes="16x16" type="image/png" /> */}

      {/* PWA config */}
      <link href="/manifest.json" rel="manifest" />
      <meta content="" name="application-name" />
      <meta content="" name="apple-mobile-web-app-title" />
      <meta content="yes" name="mobile-web-app-capable" />
      <meta content="yes" name="apple-mobile-web-app-capable" />
      <meta content="default" name="apple-mobile-web-app-status-bar-style" />
      <meta content="#ffffff" name="theme-color" />
    </Head>
  )
}

export default MetaTags
