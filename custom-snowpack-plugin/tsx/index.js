module.exports = function (snowpackConfig, pluginOptions) {
    return {
        name: '@custom-snowpack-plugin/tsx',
        resolve: {
            input: ['.js', '.jsx', '.ts', '.tsx', '.mjs'],
            output: ['.js'],
        },
        // async load({ filePath }) {},
    }
}
