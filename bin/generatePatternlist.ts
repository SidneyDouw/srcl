import fs from 'fs'
import { getRootDir } from './shared'

fs.readdir(`${getRootDir()}/src/patterns`, (err, files) => {
    fs.writeFileSync(`${getRootDir()}/src/patternlist.json`, JSON.stringify(files))
})
