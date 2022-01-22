import directiveParser from 'vite-plugin-srcl-directive-parser'
import { defineConfig, Plugin } from 'vite'
import { readFileSync } from 'fs'
import { render } from 'less'
import { ResolvedConfig } from 'vite'

export default defineConfig({
    root: './src',
    plugins: [generatePatternOutput(), directiveParser()],
    server: {
        port: 8080,
    },
    build: {
        outDir: '../dist',
        emptyOutDir: true,
    },
})

type GeneratePatternOutputOptions = {
    outputFolderName: string
}

function generatePatternOutput(options?: GeneratePatternOutputOptions): Plugin {
    let config: ResolvedConfig
    const outputFiles: { [key: string]: string } = {}

    let pluginOptions = options
        ? options
        : {
              outputFolderName: 'patternOutput',
          }

    return {
        name: 'generate-pattern-output',

        async configResolved(resolvedConfig) {
            config = resolvedConfig
        },

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
                        if (config.command === 'build') {
                            this.emitFile({
                                type: 'asset',
                                name: patternName,
                                fileName: `${pluginOptions.outputFolderName}/${patternName}/${fileName}.${fileType}`,
                                source: code,
                            })
                        } else {
                            outputFiles[`${patternName}/${fileName}.${fileType}`] = code
                        }

                        break

                    case 'less':
                        const css = await cssFromLess(
                            code,
                            basePath,
                            patternName,
                            fileName,
                            fileType,
                        )

                        if (config.command === 'build') {
                            this.emitFile({
                                type: 'asset',
                                name: patternName,
                                fileName: `${pluginOptions.outputFolderName}/${patternName}/${fileName}.${fileType}`,
                                source: code,
                            })
                            this.emitFile({
                                type: 'asset',
                                name: patternName,
                                fileName: `${pluginOptions.outputFolderName}/${patternName}/${fileName}.css`,
                                source: css,
                            })
                        } else {
                            outputFiles[`${patternName}/${fileName}.${fileType}`] = code
                            outputFiles[`${patternName}/${fileName}.css`] = css
                        }

                        break

                    case 'css':
                        if (config.command === 'build') {
                            this.emitFile({
                                type: 'asset',
                                name: patternName,
                                fileName: `${pluginOptions.outputFolderName}/${patternName}/${fileName}.${fileType}`,
                                source: code,
                            })
                        } else {
                            outputFiles[`${patternName}/${fileName}.${fileType}`] = code
                        }
                        break

                    default:
                        throw new Error(`Unknown file type: ${fileType}`)
                }
            }

            return undefined
        },

        configureServer(server) {
            server.middlewares.use((req, res, next) => {
                if (req.url?.startsWith(`/${pluginOptions.outputFolderName}/`)) {
                    let id = req.url.replace(`/${pluginOptions.outputFolderName}/`, '')

                    res.write(outputFiles[id])
                    return res.end()
                }

                next()
            })
        },
    }
}

const cssFromLess = async (
    input: string,
    basePath: string,
    patternName: string,
    fileName: string,
    fileType: string,
) => {
    const code = await render(input, {
        filename: `${basePath}/${patternName}/${fileName}.${fileType}`,
    })
    return code.css
}
