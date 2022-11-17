interface Props {
    //* label="test"

    // Basics
    string: string
    number: number
    boolean: boolean
    any: any
    literal: 'he"y' // Escaped quotation marks do not work

    // Optionals
    // stringOption?: string

    // Unions
    // TODO: they look like tuples atm, should change this
    unionPure: string | number | boolean
    unionMix: string | 'number' | boolean[]

    // Generics
    array: Array<number>
    map: Map<string, number>

    // Arrays
    stringArr: string[]
    numberArr: number[]
    booleanArr: boolean[]
    anyArr: any[]

    // Tuples

    // Objects
    // object: { a: string }
    // nestedObject: { a: string, b: { c: number } }

    // Interfaces
    // customType: CustomType
}

const Component = (props: Props) => {}

export default Component
