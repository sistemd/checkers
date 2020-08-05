const CopyPlugin = require("copy-webpack-plugin");

module.exports = {
    mode: "production",

    // Enable sourcemaps for debugging webpack's output.
    devtool: "source-map",

    resolve: {
        extensions: [".ts", ".tsx", ".js"],
    },

    module: {
        rules: [
            {
                test: /\.css$/i,
                exclude: /node_modules/,
                use: ["style-loader", "css-loader"],
            },
            {
                test: /\.ts(x?)$/,
                exclude: /node_modules/,
                use: [
                    {
                        loader: "ts-loader",
                    },
                ],
            },
            // All output '.js' files will have any sourcemaps re-processed by 'source-map-loader'.
            {
                enforce: "pre",
                test: /\.js$/,
                loader: "source-map-loader",
            },
        ],
    },

    plugins: [
        new CopyPlugin({
            patterns: [
                {
                    from: "node_modules/react/umd/react.development.js",
                    to: "./",
                },
                {
                    from: "node_modules/react-dom/umd/react-dom.development.js",
                    to: "./",
                },
            ],
        }),
    ],

    // When importing a module whose path matches one of the following, just
    // assume a corresponding global variable exists and use that instead.
    // This is important because it allows us to avoid bundling all of our
    // dependencies, which allows browsers to cache those libraries between builds.
    externals: {
        react: "React",
        "react-dom": "ReactDOM",
    },
};
