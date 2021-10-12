import React from 'react'

type Props = Partial<{
    mediaSource: string
    isVideo: boolean
    buttonStyle: 'default' | 'inverted'
}>

const defaultProps: Props = {
    mediaSource: 'https://websitedemos.net/wp-content/uploads/2019/08/hero8.jpg',
    isVideo: false,
    buttonStyle: 'default',
}

const Component = (props: Props) => {
    return (
        <section className="hero-01">
            <div>
                <div className="mediaContainer">
                    {props.isVideo ? (
                        <video className="media" src={props.mediaSource} autoPlay muted loop />
                    ) : (
                        <img className="media" src={props.mediaSource} />
                    )}
                </div>
                <div>
                    <h4> Optional Subheading </h4>
                    <h1> Main Heading You Can Edit </h1>
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

Component.defaultProps = defaultProps
export default Component
