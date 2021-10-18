import React, { lazy } from 'react'
import { useParams, useRouteMatch } from 'react-router'
import { Switch, Route } from 'react-router-dom'

export default () => {
    let { path, url } = useRouteMatch()

    return (
        <>
            <Switch>
                <Route exact path={path}>
                    <p> select a pattern </p>
                </Route>
                <Route path={`${path}/:id`}>
                    <Component />
                </Route>
            </Switch>
        </>
    )
}

const Component = () => {
    const { id } = useParams<{ id: string }>()

    const Pattern = lazy(() => import(`../../patterns/${id}/index.js`))

    return (
        <>
            <React.Suspense fallback={<p>Loading '{id}' </p>}>
                <Pattern />
            </React.Suspense>
        </>
    )
}
