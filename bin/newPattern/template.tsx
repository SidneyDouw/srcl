import React from 'react'

type Props = {}

const defaultProps: Props = {}

const Component = (props: Props) => {
    return (
        <section className="{{PATTERN_NAME}}">
            <div></div>
        </section>
    )
}

Component.defaultProps = defaultProps
export default Component
