import React from 'react'
import Test02 from '../../patterns/test-02'
import Interactive from '../../components/Interactive'

export default () => {
    fetch('/patternOutput/test-02/index.tsx')
        .then((res) => res.text())
        .then((data) => data)
        .catch((e) => console.log('could not load test-02 code'))

    return (
        <>
            <Interactive child={Test02} />
        </>
    )
}
