// Snowpack Configuration File
// See all supported options: https://www.snowpack.dev/reference/configuration

/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
    mount: {
        './src': '/',
    },
    routes: [{ match: 'routes', src: '.*', dest: '/index.html' }],
    plugins: [
        ['@custom-snowpack-plugin/less'],
        // ['@custom-snowpack-plugin/tsx']
    ],
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
