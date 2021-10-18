import React from 'react'

type Props = {
    columns: number[]
    children: JSX.Element[]
}

const defaultProps: Props = {
    columns: [50, 50],
    children: [
        <p key="1">
            Lorem ipsum dolor sit amet, consectetur adipisicing elit. Vero magni ducimus deserunt
            illo assumenda nulla facere, nisi distinctio veritatis, modi provident. Aliquam saepe
            libero doloremque mollitia dicta velit quo quia.
        </p>,
        <img key="2" src="https://i.stack.imgur.com/y9DpT.jpg" />,
    ],
}
const Component = (props: Props) => {
    return (
        <section className="layout-01">
            <div style={{ gridTemplateColumns: props.columns.join('% ') + '%' }}>
                {props.children}
            </div>
        </section>
    )
}

Component.defaultProps = defaultProps
export default Component
