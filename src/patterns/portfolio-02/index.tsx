import React from 'react'

type Props = {}

const defaultProps: Props = {}

const Component = (props: Props) => {
    return (
        <section className="portfolio-02">
            <div>
                {Array(12)
                    .fill(0)
                    .map((_, i) => (
                        <div key={i} className="pattern">
                            <img src="https://i.stack.imgur.com/y9DpT.jpg" />
                        </div>
                    ))}
            </div>
        </section>
    )
}

Component.defaultProps = defaultProps
export default Component
