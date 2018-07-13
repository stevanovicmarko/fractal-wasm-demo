const path = require('path');

module.exports = {
  entry: './assets/index.js',
  output: {
    filename: 'index.js',
    path: path.resolve(__dirname, 'assets'),
    publicPath: '/assets/',
  },
  mode: 'development',
  devServer: {
    // hot: true,
    publicPath: '/assets/',
  },
};
