import React from 'react'

type Props = {}

const defaultProps: Props = {}

const Component = (props: Props) => {
    return (
        <section className="header-01">
            <div>
                <img src="https://i.stack.imgur.com/y9DpT.jpg" />
                <nav>
                    <span> One </span>
                    <span> Two </span>
                    <span> Tree </span>
                </nav>
            </div>
        </section>
    )
}

Component.defaultProps = defaultProps
export default Component
