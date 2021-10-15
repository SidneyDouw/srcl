import fs from 'fs'
import { getRootDir } from '../shared'

const patternName = process.argv[2]
const pathToTemplate = `${getRootDir()}/bin/newPattern/template.tsx`
const pathToPatternDir = `${getRootDir()}/src/patterns/${patternName}`

if (fs.existsSync(pathToPatternDir)) {
    console.error(`Pattern with name '${patternName}' already exists`)
    process.exit(1)
}

fs.readFile(pathToTemplate, 'utf8', (err, data) => {
    if (err) {
        console.error(err)
        process.exit(1)
    }

    fs.mkdir(pathToPatternDir, (err) => {
        if (err) {
            console.error(err)
            process.exit(1)
        }

        fs.writeFile(
            `${pathToPatternDir}/index.tsx`,
            data.replace('{{PATTERN_NAME}}', patternName),
            (err) => {
                if (err) {
                    console.error(err)
                    process.exit(1)
                }
            },
        )
    })
})
