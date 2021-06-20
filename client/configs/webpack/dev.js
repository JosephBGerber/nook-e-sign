// development config
const { merge } = require("webpack-merge");
const {resolve} = require("path");
const commonConfig = require("./common");

module.exports = merge(commonConfig, {
  mode: "development",
  entry: "./index.tsx",
  output: {
    filename: "js/bundle.[contenthash].min.js",
    path: resolve(__dirname, "../../dist"),
    publicPath: "/",
  },
  devtool: "cheap-module-source-map",
});
