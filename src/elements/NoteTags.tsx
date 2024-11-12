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
import { useState } from 'react'
import { useTranslation } from 'react-i18next'

interface TagSelectorProps {
  readonly uuid: string
  readonly setOpen: (open: boolean) => void
}

function TagSelector(props: TagSelectorProps): JSX.Element {
  const [addText, setAddText] = useState('')
  const { data } = useAvailableTagsContaining(
    {},
    addText,
    BigInt(5),
    props.uuid,
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
    <div role="menu" className="bg-surface0 rounded-lg p-0.5">
      <div className="flex items-center bg-crust rounded-md m-0.5">
        <Input
          placeholder="tag"
          value={addText}
          onChange={(e) => {
            setAddText(e.target.value)
          }}
        />
      </div>
      <div className="flex flex-col">
        {options.map((option) => {
          return (
            <button
              className="m-0.5 p-1 rounded-md text-text focus:bg-surface1 focus:outline-none"
              key={option[0]}
              onClick={() => handleSelect(option[1])}
            >
              {option[0]}
            </button>
          )
        })}
      </div>
    </div>
  )
}

interface NoteTagsProps {
  readonly uuid: string
}

export function NoteTags(props: NoteTagsProps): JSX.Element {
  const { t } = useTranslation()
  const { data } = useNoteDirectTags({}, props.uuid)
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
