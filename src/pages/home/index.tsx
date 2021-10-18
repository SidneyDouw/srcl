import React, { useState } from 'react'

import patternlist from '../../patternlist.json'

import Layout01 from '../../patterns/layout-01'
import Code01 from '../../patterns/code-01'
import Portfolio02 from '../../patterns/portfolio-02'

export default () => {
    const [selectedPattern, setSelectedPattern] = useState<String>()
    const [selectedPatternCode, setSelectedPatternCode] = useState<String>()

    return (
        <>
            <Layout01>
                <Portfolio02
                    imagePaths={patternlist.map(
                        (patternname) => '/assets/images/patterns/' + patternname + '.jpg',
                    )}
                    onElementClick={(evt) => {
                        let img = evt.currentTarget.children[0] as HTMLImageElement
                        let patternname = img.src.split('/').pop()?.split('.')[0]

                        setSelectedPattern(patternname)
                        fetch(`/patterns/${patternname}/index.js`)
                            .then((res) => res.text())
                            .then((data) => setSelectedPatternCode(data))
                    }}
                />
                <Code01 code={selectedPatternCode} language="javascript" />
            </Layout01>
        </>
    )
}
