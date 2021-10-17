// https://websitedemos.net/astra-blocks/

import { render } from 'react-dom'
import React from 'react'
import { BrowserRouter as Router, Switch, Route } from 'react-router-dom'

import './index.less'

import Dev from './pages/dev'
import Home from './pages/home'
import Screenshot from './pages/screenshot'

const App = () => {
    return (
        <>
            <Router>
                <Switch>
                    <Route path="/screenshot">
                        <Screenshot />
                    </Route>

                    <Route path="/dev">
                        <Dev />
                    </Route>

                    <Route path="/">
                        <Home />
                    </Route>
                </Switch>
            </Router>
        </>
    )
}

render(<App />, document.getElementById('root'))
