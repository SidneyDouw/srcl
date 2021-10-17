import fs from 'fs'
import { getRootDir } from '../shared'

const root = getRootDir()

const patternName = process.argv[2]
const pathToTemplateTsx = `${root}/bin/newPattern/template.tsx`
const pathToTemplateLess = `${root}/bin/newPattern/template.less`
const pathToPatternDir = `${root}/src/patterns/${patternName}`
const pathToStyles = `${root}/src/index.less`

if (fs.existsSync(pathToPatternDir)) {
    console.error(`Pattern with name '${patternName}' already exists`)
    process.exit(1)
}

fs.mkdirSync(pathToPatternDir)
;[
    [pathToTemplateTsx, 'index.tsx'],
    [pathToTemplateLess, 'style.less'],
].forEach(([filepath, targetname]) => {
    fs.readFile(filepath, 'utf8', (err, data) => {
        if (err) {
            console.error(err)
            process.exit(1)
        }

        fs.writeFile(
            `${pathToPatternDir}/${targetname}`,
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

fs.appendFile(pathToStyles, `@import './patterns/${patternName}/style.less';`, (err) => {
    if (err) {
        console.error(err)
        process.exit(1)
    }
})
