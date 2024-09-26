import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext'
import {
  LexicalTypeaheadMenuPlugin,
  MenuOption,
  useBasicTypeaheadTriggerMatch,
} from '@lexical/react/LexicalTypeaheadMenuPlugin'
import { TextNode } from 'lexical'
import { useState, useMemo, useCallback, ReactPortal } from 'react'
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
  selectedIndex: number | null
}

interface LinkTypeaheadMenuItemProps {
  option: LinkTypeaheadOption
  isSelected: boolean
}

function LinkTypeaheadMenuItem(props: LinkTypeaheadMenuItemProps): JSX.Element {
  const { option, isSelected } = props
  let className = 'px-2 rounded-md py-1'

  if (isSelected === true) className += ' bg-surface1'

  return (
    <li className={className} key={option.key} tabIndex={-1}>
      {option.title}
    </li>
  )
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
      <div className="text-text w-52 p-0.5 bg-surface0 rounded-md shadow-md">
        <ul>
          {options.map((option, i: number) => (
            <LinkTypeaheadMenuItem
              option={option}
              key={option.key}
              isSelected={props.selectedIndex === i}
            />
          ))}
        </ul>
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
      menuRenderFn={(anchorElementRef, { selectedIndex }) => {
        const current = anchorElementRef.current
        if (current === null) return null
        return <MenuRenderFn target={current} selectedIndex={selectedIndex} />
      }}
    />
  )
}
