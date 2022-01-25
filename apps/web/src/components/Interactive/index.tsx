import React, { useState, createElement } from 'react'

interface UiData {
    name: string
    directives: { key: string; value: any }[]
    properties: {
        key: string
        value: any
        directives: { key: string; value: any }[]
    }[]
}

interface EasyUiData {
    name: string
    directives: { [key: string]: any }
    properties: {
        [key: string]: {
            [key: string]: any
            value: any
            directives: { [key: string]: any }
        }
    }
}

interface ParsedComponent<P> extends React.FunctionComponent<P> {
    uiData: UiData
}

interface Props {
    child: React.FunctionComponent<any>
}

export default ({ child }: Props) => {
    let c = child as ParsedComponent<any>

    const { directives, properties } = makeEasyUiData(c.uiData)
    const [state, setState] = useState(makeInitialState(properties))

    const inputs = Object.keys(properties)
        .filter((key, i) => Object.keys(properties[key].directives).length > 0)
        .map((key, i) => {
            const { directives } = properties[key]

            let inputEl: JSX.Element

            switch (directives['type']) {
                case 'checkbox':
                    inputEl = (
                        <input
                            type="checkbox"
                            checked={state[key]}
                            onChange={(e) => {
                                setState({ ...state, [key]: e.target.checked })
                            }}
                        />
                    )

                    break

                case 'select':
                    let values: string[] = properties[key].value.split(' | ')
                    let labels: string[] = directives['options']
                        ? directives['options'].split(' | ')
                        : []
                    let options = values.map((value, i) => (
                        <option key={i} value={value}>
                            {labels[i] ? labels[i] : value}
                        </option>
                    ))

                    inputEl = (
                        <select
                            value={state[key]}
                            onChange={(e) =>
                                setState({ ...state, [key]: values[e.target.selectedIndex] })
                            }
                        >
                            {options}
                        </select>
                    )

                    break

                case 'range':
                    inputEl = (
                        <input
                            type={directives['type']}
                            min={directives['min']}
                            max={directives['max']}
                            step={directives['step']}
                            value={state[key]}
                            onChange={(e) =>
                                setState({ ...state, [key]: parseFloat(e.target.value) })
                            }
                        />
                    )

                    break

                default:
                    inputEl = (
                        <input
                            type={directives['type']}
                            value={state[key]}
                            onChange={(e) => setState({ ...state, [key]: e.target.value })}
                        />
                    )
            }

            return (
                <div className={'input'} key={i}>
                    {directives['label'] ? (
                        <label>
                            {directives['label']}
                            {inputEl}
                        </label>
                    ) : (
                        inputEl
                    )}
                </div>
            )
        })

    return (
        <>
            <label>
                {directives['label']}
                {inputs}
                {createElement(c, state)}
            </label>
        </>
    )
}

const makeEasyUiData = (uiData: UiData): EasyUiData => {
    if (uiData === undefined) {
        throw new Error('UI Data not found')
    }

    const name = uiData.name

    const directives = uiData.directives.reduce((prev, d) => {
        prev[d.key] = d.value
        return prev
    }, {} as EasyUiData['directives'])

    const properties = uiData.properties.reduce((prev, p) => {
        prev[p.key] = {
            value: p.value,
            directives: p.directives.reduce((prev, d) => {
                prev[d.key] = d.value
                return prev
            }, {} as EasyUiData['properties']['directives']),
        }
        return prev
    }, {} as EasyUiData['properties'])

    return {
        name,
        directives,
        properties,
    }
}

const makeInitialState = (properties: EasyUiData['properties']) => {
    return Object.keys(properties).reduce((prev, key) => {
        const { directives } = properties[key]

        switch (directives['type']) {
            default:
                prev[key] = directives['default']
        }

        return prev
    }, {} as { [key: string]: any })
}
