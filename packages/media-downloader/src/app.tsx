import * as React from 'react'
import type { AppProps, ReactRenderContext } from '@coodev/react/types'
import type { LoaderContext } from '@/common/types'
import { Error, ErrorProps } from './components/error'
import { NotFound } from './components/not-found'
import { ToastProvider } from '@/components/ui/toast'
import {
  ProgressBarProvider,
  progressBarController,
} from '@/components/progress-bar'

type PageProps = ErrorProps | object

const Main: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  return (
    <ToastProvider>
      <main className="bg-background">{children}</main>
    </ToastProvider>
  )
}

const AppContent: React.FC<AppProps<PageProps>> = ({
  Component,
  pageProps,
}) => {
  if ('isError' in pageProps && pageProps.isError) {
    return <Error {...(pageProps as ErrorProps)} />
  }

  if (!Component) {
    return <NotFound />
  }

  return (
    <Main>
      <Component {...pageProps} />
    </Main>
  )
}

const App: React.FC<AppProps<PageProps>> = props => {
  return (
    <ProgressBarProvider>
      <AppContent {...props} />
    </ProgressBarProvider>
  )
}

App.getInitialProps = async ({ Component, ...ctx }: ReactRenderContext) => {
  try {
    progressBarController.start()

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
  } finally {
    progressBarController.complete()
  }
}

export default App
