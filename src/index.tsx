// https://websitedemos.net/astra-blocks/

import { render } from 'react-dom'
import React from 'react'

import './styles.less'

import Hero01 from './patterns/hero-01'
import Feature01 from './patterns/feature-01'
import Portfolio01 from './patterns/portfolio-01'

const App = () => {
    return (
        <>
            <Hero01 />
            <Hero01
                mediaSource="https://www.monzoom.com/static2/preview/stock-video-free-romantic-video-background-loop-0242-28603.mp4"
                isVideo
                buttonStyle="inverted"
            />

            <Feature01 />
            <Feature01 flipped />

            <Portfolio01 />
            <Portfolio01 flipped />
        </>
    )
}

render(<App />, document.getElementById('root'))
