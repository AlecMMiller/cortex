import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext'
import { HeadingTagType } from '@lexical/rich-text'
import { NodeKey } from 'lexical'
import { useRef } from 'react'

export default function PageNavigator(props: {
  tableOfContents: Array<[key: NodeKey, text: string, tag: HeadingTagType]>
}): JSX.Element {
  const [editor] = useLexicalComposerContext()
  const selectedIndex = useRef(0)
  function scrollToNode(key: NodeKey, currIndex: number): void {
    editor.getEditorState().read((): void => {
      const domElement = editor.getElementByKey(key)
      if (domElement !== null) {
        domElement.scrollIntoView()
        selectedIndex.current = currIndex
      }
    })
  }

  const entries = props.tableOfContents.map((entry, index) => {
    const key = entry[0]
    return (
      <button
        key={entry[0]}
        onClick={() => scrollToNode(key, index)}
        className="hover:text-blue w-full text-right"
      >
        {entry[1]}
      </button>
    )
  })
  return (
    <div className="text-overlay0 text-lg font-prose p-2 lg:p-9 gap-2 flex flex-col text-base font-medium min-w-56">
      {entries}
    </div>
  )
}
