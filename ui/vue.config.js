const { defineConfig } = require('@vue/cli-service')

const BASE_URL = '/dis'


module.exports = defineConfig({
  transpileDependencies: true,
  publicPath: BASE_URL

})

