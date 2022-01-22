import React from 'react'
import Feature01 from '../../patterns/feature-01'
import Interactive from '../../components/Interactive'

export default () => {
    fetch('/patternOutput/feature-01/index.tsx')
        .then((res) => res.text())
        .then((data) => data)
        .catch((e) => console.log('could not load feature-01 code'))

    return (
        <>
            <Interactive child={Feature01} />
        </>
    )
}
