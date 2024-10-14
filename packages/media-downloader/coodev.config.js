const path = require('path')

module.exports = {
  srcDir: './src',
  plugins: [
    {
      viteConfig() {
        return {
          resolve: {
            alias: {
              '@': path.resolve(__dirname, 'src')
            }
          },
        }
      }
    }
  ]
}
