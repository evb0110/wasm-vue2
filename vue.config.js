module.exports = {
  chainWebpack: config => {
    config.module
        .rule('wasm')
        .test(/\.wasm$/)
        .type('webassembly/async');
  },
  devServer: {
    headers: {
      'Cross-Origin-Embedder-Policy': 'require-corp',
      'Cross-Origin-Opener-Policy': 'same-origin'
    }
  },
  configureWebpack: {
    experiments: {
      asyncWebAssembly: true,
    },
  },
};
