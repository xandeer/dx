const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

const config = {
  entry: {
    app: './src/app.js'
  },
  output: {
    path: path.resolve(__dirname, 'dist'),
    publicPath: 'dist/',
    filename: "[name].bundle.js",
  },
    plugins: [
      new HtmlWebpackPlugin({
        template: './src/index.pug'
      }),
    ],
    module: {
      rules: [
        { 
          test: /\.pug$/,
          use: ['pug-loader']
        },
      ]
    },
  devServer: {
    host: '0.0.0.0',
    port: 3000,
  }
};

module.exports = (env, argv) => {
  if (argv.mode === 'development') {}
  if (argv.mode === 'production') {}return config;
}
