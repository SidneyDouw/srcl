import fs from 'fs'
import path from 'path'
import less from 'less'

import { SnowpackPluginFactory } from 'snowpack'

type PluginOptions = {}

const pluginFactory: SnowpackPluginFactory<PluginOptions> = () => {
    /** A map of partially resolved imports to the files that imported them. */
    const importedByMap = new Map<string, Set<string>>()

    const addImportsToMap = (filePath: string, importedFile: string) => {
        const importedBy = importedByMap.get(importedFile)
        if (importedBy) {
            importedBy.add(filePath)
        } else {
            importedByMap.set(importedFile, new Set([filePath]))
        }
    }

    return {
        name: '@custom-snowpack-plugin/less',

        resolve: {
            input: ['.less'],
            output: ['.css', '.less'],
        },

        onChange({ filePath }) {
            if (importedByMap.has(filePath)) {
                const importedBy = importedByMap.get(filePath)
                importedByMap.delete(filePath)

                importedBy?.forEach((importerFilePath) => {
                    this.markChanged!(importerFilePath)
                })
            }
        },

        async load({ filePath, isDev }) {
            const fileBuffer = await fs.promises.readFile(filePath, 'utf-8')
            const contents = fileBuffer.toString()

            let result = { '.css': { code: '' }, '.less': { code: '' } }

            // Emit Less
            if (filePath.includes('/patterns/')) {
                try {
                    result['.less'].code = emitLess(contents)
                } catch (error) {
                    throw new Error(`Error emitting LESS:\n${error}`)
                }
            }

            // Emit Css
            try {
                const output = await emitCss(contents, filePath)
                result['.css'].code = output.css

                if (isDev) {
                    output.imports.forEach((importedFile) => {
                        addImportsToMap(filePath, importedFile)
                    })
                }
            } catch (error) {
                throw new Error(`Error emitting CSS:\n${error}`)
            }

            return result
        },

        async transform({ fileExt, contents }) {
            if (this.resolve?.output.includes(fileExt)) {
                if (contents !== '') {
                    return contents !== '' ? `/* Modified */\n\n ${contents}` : ''
                }
            }
        },

        async run({ isDev }) {
            console.log('run command', isDev)
        },

        async optimize({ buildDirectory }) {
            const files = getAllFiles(path.join(buildDirectory, 'patterns')).filter((file) =>
                this.resolve!.output.some((ext) => file.endsWith(ext)),
            )

            files.forEach((file) => {
                const newName = file.replace('.less', '')
                fs.renameSync(file, newName)
            })
        },
    }
}

export default pluginFactory

const emitLess = (input: string): string => {
    return input
}

const emitCss = async (input: string, filePath: string) => {
    return await less.render(input, {
        filename: filePath,
    })
}

const getAllFiles = (dirPath: string, arrayOfFiles: string[] = []) => {
    const files = fs.readdirSync(dirPath)

    files.forEach((file) => {
        if (fs.statSync(dirPath + '/' + file).isDirectory()) {
            arrayOfFiles = getAllFiles(dirPath + '/' + file, arrayOfFiles)
        } else {
            arrayOfFiles.push(path.join(dirPath, '/', file))
        }
    })

    return arrayOfFiles
}
