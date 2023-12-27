module.exports = {
  devServer: {
    port: 8081,
    proxy: {
      '/process_csv': {
        target: 'http://localhost:8080',
        changeOrigin: true,
      },
    },
  },
  transpileDependencies: true,
};
