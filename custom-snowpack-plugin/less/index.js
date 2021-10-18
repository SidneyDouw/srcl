const fs = require('fs')
const less = require('less')

module.exports = function (snowpackConfig, pluginOptions) {
    return {
        name: '@custom-snowpack-plugin/less',

        resolve: {
            input: ['.less'],
            output: ['.css', '.less'],
        },

        async load({ filePath }) {
            const contents = (await fs.promises.readFile(filePath, 'utf-8')).toString()

            let result = { '.css': { code: '' }, '.less': { code: '' } }

            if (filePath.includes('/patterns/')) {
                result['.less'].code = contents
            }

            try {
                result['.css'].code = (
                    await less.render(contents, {
                        filename: filePath,
                    })
                ).css
            } catch (error) {
                console.log(error)
            }

            return result
        },

        async transform({ id, contents, isDev, fileExt }) {
            console.log(id)
        },
    }
}
