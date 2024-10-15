import * as React from 'react'
import type { AppProps, ReactRenderContext } from '@coodev/react/types'
import type { LoaderContext } from '@/common/types'

interface ErrorProps {
  isError: boolean
  error: {
    message: string
    stack: string
  }
}

type PageProps = ErrorProps | object

const Main: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  return <main className="bg-background">{children}</main>
}

const Error: React.FC<ErrorProps> = ({ error }) => {
  return (
    <Main>
      <div>
        <h1>Error</h1>
        <p>{error.message}</p>
        <pre>{error.stack}</pre>
      </div>
    </Main>
  )
}

const App: React.FC<AppProps<PageProps>> = ({ Component, pageProps }) => {
  if ('isError' in pageProps && pageProps.isError) {
    return <Error {...(pageProps as ErrorProps)} />
  }

  if (!Component) {
    return (
      <Main>
        <h1 className="font-bold text-center text-lg p-8">Page not found</h1>
      </Main>
    )
  }

  return (
    <Main>
      <Component {...pageProps} />
    </Main>
  )
}

App.getInitialProps = async ({ Component, ...ctx }: ReactRenderContext) => {
  try {
    if (Component && Component.getInitialProps) {
      const loaderContext: LoaderContext = ctx

      return await Component.getInitialProps(loaderContext)
    }

    return {}
  } catch (error) {
    const typedError = error as Error
    return {
      isError: true,
      error: {
        message: typedError.message,
        stack: typedError.stack,
      },
    }
  }
}

export default App
