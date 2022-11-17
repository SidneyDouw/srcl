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
            <Interactive child={Test02}>
                <div> 1 </div>
                <div> 2 </div>
                <div> 3 </div>
                <div> 4 </div>
                <div> 5 </div>
            </Interactive>
        </>
    )
}
