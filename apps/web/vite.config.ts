import directiveParser from 'vite-plugin-srcl-directive-parser'
import { defineConfig, Plugin } from 'vite'
import { PluginContext } from 'rollup'
import { readFileSync } from 'fs'
import { render } from 'less'

export default defineConfig({
    root: './src',
    plugins: [patternCode(), directiveParser()],
    server: {
        port: 8080,
    },
    build: {
        outDir: '../dist',
        emptyOutDir: true,
    },
})

function patternCode(): Plugin {
    return {
        name: 'pattern-code',
        apply: 'build',

        async load(id, options) {
            if (id.includes('src/patterns/')) {
                let basePath = id.match(/(.*\/src\/patterns)/)![1]
                let patternName = id.match(/src\/patterns\/(.*)\/.*/)![1]
                let fileName = id.match(/src\/patterns\/.*\/(.*)\./)![1]
                let fileType = id.match(/src\/patterns\/.*\.(\w*)/)![1]

                let code = readFileSync(
                    `${basePath}/${patternName}/${fileName}.${fileType}`,
                    'utf8',
                ).toString()

                switch (fileType) {
                    case 'tsx':
                        emitInput(this, code, patternName, fileName, fileType)
                        break

                    case 'less':
                        emitInput(this, code, patternName, fileName, fileType)
                        await emitCssFromLess(this, code, basePath, patternName, fileName, fileType)

                        break

                    case 'css':
                        emitInput(this, code, patternName, fileName, fileType)
                        break

                    default:
                        throw new Error(`Unknown file type: ${fileType}`)
                }
            }

            return undefined
        },
    }
}

const emitInput = (
    context: PluginContext,
    input: string,
    patternName: string,
    fileName: string,
    fileType: string,
) => {
    context.emitFile({
        type: 'asset',
        name: patternName,
        fileName: `patterns/${patternName}/${fileName}.${fileType}`,
        source: input,
    })
}

const emitCssFromLess = async (
    context: PluginContext,
    input: string,
    basePath: string,
    patternName: string,
    fileName: string,
    fileType: string,
) => {
    const code = await render(input, {
        filename: `${basePath}/${patternName}/${fileName}.${fileType}`,
    })

    context.emitFile({
        type: 'asset',
        name: patternName,
        fileName: `patterns/${patternName}/${fileName}.css`,
        source: code.css,
    })
}
