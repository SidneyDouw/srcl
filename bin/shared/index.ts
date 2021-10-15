import fs from 'fs'
import path from 'path'

export const getRootDir = () => {
    for (let p of module.paths) {
        if (fs.existsSync(p)) {
            return path.dirname(p)
        }
    }

    throw new Error('Could not find project root directory')
}
