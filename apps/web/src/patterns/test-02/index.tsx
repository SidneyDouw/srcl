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
                        return (
                            <React.Fragment key={i}>
                                {props.children && props.children[i] ? (
                                    props.children[i]
                                ) : (
                                    <div> empty </div>
                                )}
                            </React.Fragment>
                        )
                    })}
            </div>
        </section>
    )
}

export default Component
