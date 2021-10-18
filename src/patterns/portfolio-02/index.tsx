import React from 'react'

type Props = {
    imagePaths: string[]
    onElementClick: (evt: React.MouseEvent) => void
}

const defaultProps: Props = {
    imagePaths: [
        'https://i.stack.imgur.com/y9DpT.jpg',
        'https://i.stack.imgur.com/y9DpT.jpg',
        'https://i.stack.imgur.com/y9DpT.jpg',
        'https://i.stack.imgur.com/y9DpT.jpg',
    ],
    onElementClick: () => {},
}

const Component = (props: Props) => {
    return (
        <section className="portfolio-02">
            <div>
                {props.imagePaths.map((path, i) => (
                    <div key={i} className="pattern" onClick={props.onElementClick}>
                        <img src={path} />
                    </div>
                ))}
            </div>
        </section>
    )
}

Component.defaultProps = defaultProps
export default Component
