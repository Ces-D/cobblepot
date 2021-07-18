const path=require('path')

module.exports = {
  reactStrictMode: true,
  sassOptions: {
    includePath: [path.join(__dirname, 'styles')]
  }
}
