import { HeadingTagType } from '@lexical/rich-text'
import { NodeKey } from 'lexical'
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext'
import { useRef } from 'react'

export type TocContents = Array<
  [key: NodeKey, text: string, tag: HeadingTagType]
>
interface TableOfContentsProps {
  readonly toc: TocContents
}

export function TableOfContentsNavigator(
  props: TableOfContentsProps,
): JSX.Element {
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

  const entries = props.toc.map((entry, index) => {
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
    <div className="text-overlay0 text-lg font-prose p-2 lg:p-9 gap-2 flex flex-col text-base font-medium">
      {entries}
    </div>
  )
}
