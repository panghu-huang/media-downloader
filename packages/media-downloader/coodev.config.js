const path = require('path')

const nodeEnv = process.env.NODE_ENV || 'development'

if (nodeEnv === 'development') {
  require('dotenv').config()
}

module.exports = {
  srcDir: './src',
  runtimeConfig: {
    apiBaseURL: process.env.API_BASE_URL,
    apiBaseURLServer: process.env.API_BASE_URL_SERVER || process.env.API_BASE_URL
  },
  plugins: [
    {
      viteConfig() {
        return {
          resolve: {
            alias: {
              '@': path.resolve(__dirname, 'src'),
            },
          },
        }
      },
    },
  ],
}
