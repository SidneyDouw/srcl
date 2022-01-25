import React from 'react'

//* label="test-02"
interface Props {
    //* label="Columns" type="range"
    //* step=1 min=1 max=6
    //* default=4
    columns: number

    children: JSX.Element[]
}

const Component = (props: Props) => {
    return (
        <section className="test-02">
            <div>
                {Array(props.columns)
                    .fill(0)
                    .map((_, i) => {
                        // return props.children[i]
                        return <p key={i}> Column {i} </p>
                    })}
            </div>
        </section>
    )
}

export default Component
