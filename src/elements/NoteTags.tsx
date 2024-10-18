import { addNewTag, addTag, useNoteDirectTags } from '@/commands/note'
import { useAvailableTagsContaining } from '@/commands/tags'
import { Input } from '@/components/ui/input'
import {
  Popover,
  PopoverTrigger,
  PopoverContent,
} from '@/components/ui/popover'
import { useQueryClient } from '@tanstack/react-query'
import { CirclePlus } from 'lucide-react'
import { KeyboardEventHandler, useState } from 'react'
import { useTranslation } from 'react-i18next'

interface TagSelectorProps {
  readonly uuid: string
  readonly setOpen: (open: boolean) => void
}

function TagSelector(props: TagSelectorProps): JSX.Element {
  const [addText, setAddText] = useState('')
  const [selected, setSelected] = useState(-1)
  const { data } = useAvailableTagsContaining(
    { content: addText, maxResults: 5, noteUuid: props.uuid },
    {},
  )
  const { t } = useTranslation()
  const qc = useQueryClient()

  console.log(data)

  const options: Array<[string, string | undefined]> = []

  if (data !== undefined) {
    data[0].forEach((datum) => {
      options.push([datum.title, datum.uuid])
    })
  }

  const exists = data ? data[1] : true

  if (addText !== '' && !exists) {
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
    props.setOpen(false)
  }

  return (
    <div onKeyDown={handleKeyDown} className="bg-surface0 rounded-lg p-0.5">
      <div className="flex items-center bg-crust rounded-md m-0.5">
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
          let className = index === selected ? 'bg-surface1' : 'bg-surface0'
          className += ' m-0.5 p-1 hover:cursor-pointer rounded-md text-text'
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
  readonly uuid: string
}

export function NoteTags(props: NoteTagsProps): JSX.Element {
  const { t } = useTranslation()
  const { data } = useNoteDirectTags({ uuid: props.uuid }, {})
  const [open, setOpen] = useState(false)

  const tagData = data ?? []

  const tags = (
    <div className="flex gap-2 flex-wrap p-2 justify-center">
      {tagData.map((tag) => {
        return (
          <div
            key={tag.uuid}
            className="bg-surface0 rounded-sm p-1 px-2 text-sm text-text"
          >
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
        <Popover open={open} onOpenChange={setOpen}>
          <PopoverTrigger>
            <CirclePlus size={16} />
          </PopoverTrigger>
          <PopoverContent>
            <TagSelector setOpen={setOpen} uuid={props.uuid} />
          </PopoverContent>
        </Popover>
      </div>
    </div>
  )
}
