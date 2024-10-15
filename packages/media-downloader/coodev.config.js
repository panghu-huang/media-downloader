const path = require('path')

module.exports = {
  srcDir: './src',
  runtimeConfig: {
    apiBaseURL: 'http://192.168.2.105:5231/api/v1',
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
