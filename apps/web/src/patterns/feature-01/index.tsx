import React from 'react'
import './styles.less'

interface Props {
    //* label="Media Source"
    //* default="https://websitedemos.net/wp-content/uploads/2020/01/hero9-bg.jpg"
    mediaSource: string

    //* label="Button Style" type="select"
    //* default="inverted"
    // TODO: Individual labels for the options
    buttonStyle: 'default' | 'inverted'

    //* label="Flipped" type="checkbox"
    // TODO: Is being parsed as a string, should maybe be parsed as a boolean
    //* default="false"
    flipped: boolean
}

const Component = (props: Props) => {
    return (
        <section className="feature-01">
            <div className={props.flipped ? 'flipped' : ''}>
                <img className="media" src={props.mediaSource} />
                <div>
                    <h2> Main Heading You Can Edit </h2>
                    <p>
                        Lorem, ipsum dolor sit amet consectetur adipisicing elit. Debitis officiis
                        repudiandae labore iusto voluptas corrupti quaerat aspernatur vero minima,
                        impedit pariatur laborum similique, quasi ut totam est itaque placeat porro.
                    </p>
                    <button className={props.buttonStyle}> Call to Action </button>
                </div>
            </div>
        </section>
    )
}

export default Component
