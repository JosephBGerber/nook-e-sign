// shared config (dev and prod)
const {resolve} = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
    resolve: {
        extensions: [".js", ".jsx", ".ts", ".tsx", ".css"],
    },
    context: resolve(__dirname, "../../src"),
    module: {
        rules: [
            {
                test: [/\.tsx?$/],
                use: ["ts-loader"],
                exclude: /node_modules/,
            },
            {
                test: [/\.jsx?$/],
                use: ["babel-loader"],
                exclude: /node_modules/,
            },
            {
                test: /\.css$/,
                use: ["style-loader", "css-loader"],
            },
        ],
    },
    performance: {
        hints: false,
    },
    plugins: [new HtmlWebpackPlugin({
        title: "Nook-E-Sign",
        template: "index.html.ejs"
    })],
};
