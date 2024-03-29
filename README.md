# Simple React Component Library

The goal of this project is for users to be able to browse a library of components, that can be interactively adjusted to the users specific needs. These parameters are defined by the component author through its `Props` type. Once the user is happy with the adjustments they can copy the code and use it in their own project.

We start with defining a type for the components `Props` that has some `Directive Syntax` sprinkled around it:

```tsx
type Props = {
	//* type=range min=0 max=99 step=1 default=0
	//* label=Number
    num: number
}
```

And we define a component that uses those `Props`:

```tsx
const Component = (props: Props) => {
    return (
        <section className="simple-component">
			<div> {props.num} </div>
        </section>
    )
}
```

When users of the library navigate to any component, they should see the `Props` interface displayed as modifiable inputs, and a live preview of the component which updates in realtime with changes those the inputs.

This enables the user to customize the component to their liking and then just copy the code to it in their own project.

To enable the `Props` to be transformed into UI, Directive Syntax is used that will be ignored by the typescript compiler, but picked up by our custom parser.

## Directive Syntax

Directive snytax is used to give additional properties to the underlying expression.

Directives can be declared for an entire interface:

```typescript
//* label=Anything 
interface Something {
	...
}
```

Additionally you can add them to properties inside the interface:

```typescript
interface Something {
	//* type=checkbox
	prop: boolean
}
```

## Parser

Its the parser's job to pick up [[#Directive Syntax]] and parse it into a data structure that we can then convert in to a regular Javascript Object and inject into the component, from where the Transformer can pick it up to build a UI from it.

#TODO 
- [x] Define the syntax 
- [x] Lexer - turn the input into tokens
- [ ] Parser - parse groups of tokens into expression structs 
	- [ ] Find a good format for the [[Parser Output Format|parser output]]
	- [ ] Setup some unit tests
- [ ] Transformer - Take the [[Parser Output Format|parser output]] and transform it into ...
a javascript object maybe? or directly into html, or maybe react, we shall see...

### Parser Output Format

Final type of the javascript object:

```typescript
interface UiData {
	name: string
	directives: { key: string, value: any }[]
	properties: {
		key: string,
		value: any,
		directives: { key: string, value: any }[]
	}[]
}
```

Direct parser output

```rust
struct Interface {
	name: String,
	directives: Vec<Directive>,
	properties: Vec<InterfaceProperty>
}

enum PropertyValue {
	String(String),
	Number(String),
	Interface(Interface)
}

struct InterfaceProperty {
	key: String,
	value: PropertyValue,
	directives: Vec<Directive>
}

enum DirectiveValue {
	String(String),
	Number(String)
}

struct Directive {
	key: String,
	value: DirectiveValue
}
```

## Transformer

The transformer takes both the original component and the javascript object we recieve from the Parser and transforms it into a bunch of HTML inputs that feed into the component.

This behaviour is encapsulated in another component that is exposed to the user, so they can decide which components should have an interactive UI.

```tsx
interface Props {
	uiData: UiData
	children: JSX.Element
}

const MakeInteractive = ({uiData, children}: Props) => {
	
	const { properties } = makeEasyUiData(uiData)
    const [state, setState] = useState(makeInitialState(properties))
    const inputs = makeInputs(properties)

    return (
        <>
            {inputs}
            {cloneElement(children, state)}
        </>
    )
}
```
