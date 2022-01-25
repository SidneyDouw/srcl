interface Props {
    //* type="range" min=0 max=99 step=1 default=0
    //* label="Number"
    num: number

    //* type="text"
    //* default="https://websitedemos.net/wp-content/uploads/2020/01/hero8-bg.jpg"
    string: 'sometthing' | 'else' | string

    //* label="Flipped" type="checkbox"
    //* default=true
    flipped: boolean

    children: JSX.Element[]
}

const Component = (props: Props) => {}

export default Component
