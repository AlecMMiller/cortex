import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext'
import {
  LexicalTypeaheadMenuPlugin,
  MenuOption,
  QueryMatch,
} from '@lexical/react/LexicalTypeaheadMenuPlugin'
import { TextNode } from 'lexical'
import { useState, useMemo, useCallback, ReactPortal } from 'react'
import { createPortal } from 'react-dom'
import { useSearchNotesByTitle } from '@/commands/note'
import { NoteTitle } from '@/types'
import { $createInternalLinkNode } from './InternalLink'

const SUGGESTION_LIST_LENGTH_LIMIT = 5

class LinkTypeaheadOption extends MenuOption {
  uuid: string
  title: string

  constructor(note: NoteTitle) {
    super(note.title)
    this.uuid = note.uuid
    this.title = note.title
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
    <li className={className} key={option.uuid} tabIndex={-1}>
      {option.title}
    </li>
  )
}

const LENGTH_LIMIT = 20

const linkRegex = new RegExp(
  '(^|\\s|\\()(' +
    '\\[{2}' +
    '((?:' +
    '[^\\[\\]]' +
    '){0,' +
    LENGTH_LIMIT +
    '})' +
    ')$',
)

function checkForLinks(text: string): QueryMatch | null {
  const match = linkRegex.exec(text)

  if (match === null) return null

  const maybeLeadingWhitespace = match[1]
  const matchingString = match[2]

  return {
    leadOffset: match.index + maybeLeadingWhitespace.length,
    matchingString,
    replaceableString: matchingString,
  }
}

export default function WikiLinkPlugin(): JSX.Element {
  const [editor] = useLexicalComposerContext()

  const checkForLinkMathch = useCallback(
    (text: string) => {
      return checkForLinks(text)
    },
    [editor],
  )

  const [queryString, setQueryString] = useState<string | null>(null)

  const actualQueryString = queryString === null ? '' : queryString.slice(2)

  const { data, status } = useSearchNotesByTitle(
    {
      title: actualQueryString,
    },
    {},
  )

  const results = status === 'success' ? data : []

  const options = useMemo(
    () =>
      results
        .map((result) => new LinkTypeaheadOption(result))
        .slice(0, SUGGESTION_LIST_LENGTH_LIMIT),
    [results],
  )

  const onSelectOption = useCallback(
    (
      selectedOption: LinkTypeaheadOption,
      nodeToReplace: TextNode | null,
      closeMenu: () => void,
    ) => {
      console.log('Doing a thing')
      editor.update(() => {
        const linkNode = $createInternalLinkNode(
          selectedOption.uuid,
          selectedOption.title,
        )

        if (nodeToReplace) {
          console.log(nodeToReplace)
          nodeToReplace.replace(linkNode)
        }
        linkNode.select()
        closeMenu()
      })
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
      triggerFn={checkForLinkMathch}
      options={options}
      menuRenderFn={(anchorElementRef, { selectedIndex }) => {
        const current = anchorElementRef.current
        if (current === null) return null
        return <MenuRenderFn target={current} selectedIndex={selectedIndex} />
      }}
    />
  )
}
