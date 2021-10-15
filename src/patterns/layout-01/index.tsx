import React from 'react'

type Props = {
    children: JSX.Element[]
}

const defaultProps: Props = {
    children: [
        <p>
            Lorem ipsum dolor sit amet, consectetur adipisicing elit. Vero magni ducimus deserunt
            illo assumenda nulla facere, nisi distinctio veritatis, modi provident. Aliquam saepe
            libero doloremque mollitia dicta velit quo quia.
        </p>,
        <img src="https://i.stack.imgur.com/y9DpT.jpg" />,
    ],
}
const Component = (props: Props) => {
    return (
        <section className="layout-01">
            <div>{props.children}</div>
        </section>
    )
}

Component.defaultProps = defaultProps
export default Component
