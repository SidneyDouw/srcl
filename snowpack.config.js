// Snowpack Configuration File
// See all supported options: https://www.snowpack.dev/reference/configuration

/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
    mount: {
        './src': '/',
    },
    plugins: ['snowpack-plugin-less'],
    packageOptions: {
        /* ... */
    },
    devOptions: {
        /* ... */
    },
    buildOptions: {
        out: './dist',
        clean: true,
    },
}
