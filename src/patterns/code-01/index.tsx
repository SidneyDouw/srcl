import React, { useEffect } from 'react'
import Prism from 'prismjs'

import 'prismjs/themes/prism.css'
import 'prismjs/plugins/line-numbers/prism-line-numbers.css'
import 'prismjs/plugins/toolbar/prism-toolbar.css'

import 'prismjs/components/prism-rust'
import 'prismjs/components/prism-typescript'

import 'prismjs/plugins/line-numbers/prism-line-numbers'
import 'prismjs/plugins/toolbar/prism-toolbar'
import 'prismjs/plugins/copy-to-clipboard/prism-copy-to-clipboard'

type Props = {}

const defaultProps: Props = {}

const Component = (props: Props) => {
    useEffect(() => {
        Prism.highlightAll()
    })

    return (
        <section className="code-01">
            <div
                className="line-numbers copy-to-clipboard-button"
                style={{
                    backgroundColor: '#333',
                    height: 'calc(100% - 40px)',
                    padding: '20px',
                }}
            >
                <pre>
                    <code className="language-typescript">{`console.log('hello world')`}</code>
                </pre>
                <pre>
                    <code className="language-rust">{`println!("hello world");`}</code>
                </pre>
                <pre>
                    <code className="language-rust">{`fn main() {
    panic!("something went wrong");
}`}</code>
                </pre>
            </div>
        </section>
    )
}

Component.defaultProps = defaultProps
export default Component
