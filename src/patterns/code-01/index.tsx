import React, { useEffect } from 'react'
import Prism from 'prismjs'

import 'prismjs/themes/prism.css'

import 'prismjs/plugins/toolbar/prism-toolbar.css'
import 'prismjs/plugins/line-numbers/prism-line-numbers'

import 'prismjs/plugins/line-numbers/prism-line-numbers.css'
import 'prismjs/plugins/toolbar/prism-toolbar'

import 'prismjs/plugins/copy-to-clipboard/prism-copy-to-clipboard'

import 'prismjs/components/prism-typescript'

type Props = {
    code: String
    language: String
}

const defaultProps: Props = {
    code: "console.log('hey')",
    language: 'javascript',
}

const Component = (props: Props) => {
    useEffect(() => {
        Prism.highlightAll()
    })

    return (
        <section className="code-01">
            <div className="line-numbers copy-to-clipboard-button">
                <pre>
                    <code className={`language-${props.language}`}>{props.code}</code>
                </pre>
            </div>
        </section>
    )
}

Component.defaultProps = defaultProps
export default Component
