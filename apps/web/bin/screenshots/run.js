const { openBrowser, goto, screenshot, closeBrowser, waitFor, $ } = require('taiko')
const fs = require('fs')
const rootDir = require('../shared').getRootDir()

const main = async (patternName) => {
    await goto(`localhost:8080/screenshot/${patternName}`)
    await waitFor($('section'))
    await waitFor(250)
    await screenshot($('section'), {
        path: `${rootDir}/src/assets/images/patterns/${patternName}.jpg`,
    })
}

;(async () => {
    const patternNames = JSON.parse(await fs.promises.readFile(`${rootDir}/src/patternlist.json`))

    await openBrowser()
    for (let patternName of patternNames) {
        await main(patternName)
    }
    await closeBrowser()
})()
