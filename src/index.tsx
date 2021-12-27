// https://websitedemos.net/astra-blocks/

import { render } from 'react-dom'
import React from 'react'
import { BrowserRouter as Router, Switch, Route } from 'react-router-dom'

import './index.less'

import Home from './pages/Home'
import LibComponent from './pages/LibComponent'

const App = () => {
    return (
        <>
            <Router>
                <Switch>
                    <Route path="/screenshot">
                        <LibComponent />
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
