const path = require('path')

const nodeEnv = process.env.NODE_ENV || 'development'

if (nodeEnv === 'development') {
  require('dotenv').config()
}

const defaultAPIUrl = 'http://192.168.2.105:5231/api/v1'

module.exports = {
  srcDir: './src',
  runtimeConfig: {
    apiBaseURL: process.env.API_BASE_URL || defaultAPIUrl,
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
