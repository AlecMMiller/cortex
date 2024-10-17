import { addNewTag, addTag, useNoteDirectTags } from '@/commands/note'
import { useTagsContaining } from '@/commands/tags'
import { Input } from '@/components/ui/input'
import {
  Popover,
  PopoverTrigger,
  PopoverContent,
} from '@/components/ui/popover'
import { useQueryClient } from '@tanstack/react-query'
import { CirclePlus, Tag } from 'lucide-react'
import { KeyboardEventHandler, useState } from 'react'
import { useTranslation } from 'react-i18next'

interface TagSelectorProps {
  uuid: string
}

function TagSelector(props: TagSelectorProps): JSX.Element {
  const [addText, setAddText] = useState('')
  const [selected, setSelected] = useState(-1)
  const { data } = useTagsContaining({ content: addText, maxResults: 5 }, {})
  const { t } = useTranslation()
  const qc = useQueryClient()

  const options: Array<[string, string | undefined]> = []

  if (data !== undefined) {
    data.forEach((datum) => {
      options.push([datum.title, datum.uuid])
    })
  }

  const first = data ? data[0]?.title.toLowerCase() : ''

  if (addText !== '' && addText.toLowerCase() !== first) {
    options.push([t('add_value', { value: addText }), undefined])
  }

  const handleKeyDown: KeyboardEventHandler = (e) => {
    const key = e.key
    if (key === 'ArrowDown') {
      setSelected(
        selected < options.length - 1 ? selected + 1 : options.length - 1,
      )
    } else if (key === 'ArrowUp') {
      setSelected(selected >= 0 ? selected - 1 : -1)
    } else if (key === 'Enter') {
      const selectedOption = options[selected]

      if (selectedOption !== undefined) {
        handleSelect(selectedOption[1])
      }
    } else if (key === 'Tab') {
      if (selected < options.length - 1) {
        setSelected(selected + 1)
      } else {
        setSelected(-1)
      }
    }
  }

  const handleSelect = async (uuid: string | undefined) => {
    if (uuid === undefined) {
      console.log(`Creating new tag ${addText}`)
      await addNewTag(props.uuid, addText, qc)
    } else {
      console.log(`Adding tag ${uuid}`)
      await addTag(props.uuid, uuid, qc)
    }
  }

  return (
    <div onKeyDown={handleKeyDown} className="bg-surface0 rounded-lg">
      <div className="flex items-center bg-crust">
        <Tag size={16} className="text-subtext0" />
        <Input
          placeholder="tag"
          value={addText}
          onChange={(e) => {
            setAddText(e.target.value)
          }}
        />
      </div>
      <ul
        onMouseLeave={() => {
          setSelected(-1)
        }}
      >
        {options.map((option, index) => {
          let className = index === selected ? 'bg-surface0' : 'bg-surface1'
          className += ' hover:cursor-pointer'
          return (
            <li
              className={className}
              key={option[0]}
              onMouseOver={() => setSelected(index)}
              onClick={() => handleSelect(option[1])}
            >
              {option[0]}
            </li>
          )
        })}
      </ul>
    </div>
  )
}

interface NoteTagsProps {
  uuid: string
}

export function NoteTags(props: NoteTagsProps): JSX.Element {
  const { t } = useTranslation()
  const { data } = useNoteDirectTags({ uuid: props.uuid }, {})

  const tagData = data ?? []

  const tags = (
    <div className="flex gap-2 flex-wrap p-2 justify-center">
      {tagData.map((tag) => {
        return (
          <div className="bg-surface1 rounded-sm p-1 px-2 text-sm text-text">
            {tag.title}
          </div>
        )
      })}
    </div>
  )

  const body =
    tagData.length > 0 ? (
      tags
    ) : (
      <div className="mt-6 text-center w-full text-subtext0">
        {t('No Tags')}
      </div>
    )

  return (
    <div className="flex flex-col gap-4">
      {body}
      <div className="flex w-full justify-center text-subtext0">
        <Popover>
          <PopoverTrigger>
            <CirclePlus size={16} />
          </PopoverTrigger>
          <PopoverContent>
            <TagSelector uuid={props.uuid} />
          </PopoverContent>
        </Popover>
      </div>
    </div>
  )
}
