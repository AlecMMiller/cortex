import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext'
import {
  LexicalTypeaheadMenuPlugin,
  MenuOption,
  MenuRenderFn,
  useBasicTypeaheadTriggerMatch,
} from '@lexical/react/LexicalTypeaheadMenuPlugin'
import { TextNode } from 'lexical'
import {
  useState,
  useMemo,
  useCallback,
  ReactPortal,
  MutableRefObject,
} from 'react'
import { createPortal } from 'react-dom'

const SUGGESTION_LIST_LENGTH_LIMIT = 5

function useNoteTitleLookup(_partial: string | null): string[] {
  return ['test1', 'test2', 'test3']
}

class LinkTypeaheadOption extends MenuOption {
  title: string

  constructor(title: string) {
    super(title)
    this.title = title
  }
}

interface MenuRenderFnProps {
  target: HTMLElement
}

export default function WikiLinkPlugin(): JSX.Element {
  const [editor] = useLexicalComposerContext()

  const [queryString, setQueryString] = useState<string | null>(null)

  const results = useNoteTitleLookup(queryString)

  const options = useMemo(
    () =>
      results
        .map((result) => new LinkTypeaheadOption(result))
        .slice(0, SUGGESTION_LIST_LENGTH_LIMIT),
    [results],
  )

  const onSelectOption = useCallback(
    (
      _selectedOption: LinkTypeaheadOption,
      _nodeToReplace: TextNode | null,
      closeMenu: () => void,
    ) => {
      closeMenu()
    },
    [editor],
  )

  const MenuRenderFn = (props: MenuRenderFnProps): ReactPortal => {
    return createPortal(
      <div className="text-text w-52 p-2 bg-surface0 rounded-md shadow-md">
        Test
      </div>,
      props.target,
    )
  }

  return (
    <LexicalTypeaheadMenuPlugin<LinkTypeaheadOption>
      onQueryChange={setQueryString}
      onSelectOption={onSelectOption}
      triggerFn={useBasicTypeaheadTriggerMatch('[', {})}
      options={options}
      menuRenderFn={(anchorElementRef) => {
        const current = anchorElementRef.current
        if (current === null) return null
        return <MenuRenderFn target={current} />
      }}
    />
  )
}
