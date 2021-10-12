import React from 'react'

type Props = Partial<{
    flipped: boolean
}>

const defaultProps: Props = {
    flipped: false,
}

const Component = (props: Props) => {
    return (
        <section className="portfolio-01">
            <div className={props.flipped ? 'flipped' : ''}>
                <h2> This is a secondary heading </h2>
                <p>
                    Lorem ipsum dolor sit amet consectetur adipisicing elit. Dolore architecto iure
                    sint. Saepe vitae adipisci laudantium. Nihil blanditiis consequatur, placeat
                    consectetur ducimus distinctio facere modi, odit excepturi sit unde fugit!
                </p>
                <div className="mediaContainer">
                    <img src="https://websitedemos.net/wp-content/uploads/2019/08/hero8.jpg" />
                    <div>
                        <img src="https://websitedemos.net/wp-content/uploads/2019/08/hero8.jpg" />
                        <img src="https://websitedemos.net/wp-content/uploads/2019/08/hero8.jpg" />
                        <img src="https://websitedemos.net/wp-content/uploads/2019/08/hero8.jpg" />
                        <img src="https://websitedemos.net/wp-content/uploads/2019/08/hero8.jpg" />
                    </div>
                </div>
            </div>
        </section>
    )
}

Component.defaultProps = defaultProps
export default Component
