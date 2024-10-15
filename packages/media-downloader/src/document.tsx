import * as React from 'react'
import { Html, Head, Main, CoodevScript } from '@coodev/react/document'

const Document: React.FC = () => {
  return (
    <Html lang="en">
      <Head>
        <meta charSet="utf-8" />
        <title>Media Downloader</title>
        <link rel="stylesheet" href="/globals.css" />
      </Head>
      <body>
        <Main />
        <CoodevScript />
      </body>
    </Html>
  )
}

export default Document
