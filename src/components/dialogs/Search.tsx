import { useTranslation } from 'react-i18next'
import { DialogContent } from '../ui/dialog'
import { DialogFunctionProps } from '../ui/nav-button'
import { Search } from 'lucide-react'
import { useEffect, useState } from 'react'
import { TitleWithContext, useSearchNotesByContent } from '@/commands/note'
import { NoteLink } from '../ui/note-link'

interface SearchResultProps {
  readonly note: TitleWithContext
  readonly setOpen: (open: boolean) => void
}

function SearchResult(props: SearchResultProps): JSX.Element {
  return (
    <div className="flex flex-col gap-1">
      <NoteLink
        className="text-xl"
        onClick={() => {
          props.setOpen(false)
        }}
        note={props.note.title}
      />
      <div
        className="text-sm"
        dangerouslySetInnerHTML={{ __html: props.note.context }}
      />
    </div>
  )
}

export function SearchDialog(props: DialogFunctionProps) {
  const [queryPhrase, setQueryPhrase] = useState('')
  const [cachedResults, setCachedResults] = useState<TitleWithContext[]>([])
  const { t } = useTranslation()

  const { data } = useSearchNotesByContent(
    {},
    queryPhrase,
    BigInt(10),
    BigInt(40),
  )

  useEffect(() => {
    setCachedResults(data ?? [])
  }, [data])

  const results = cachedResults.map((result) => (
    <SearchResult
      setOpen={props.setOpen}
      key={result.title.uuid}
      note={result}
    />
  ))

  return (
    <DialogContent>
      <div className="flex items-center gap-2">
        <Search size={18} />
        <input
          value={queryPhrase}
          onChange={(e) => setQueryPhrase(e.target.value)}
          placeholder={t('Search Notes')}
          className="bg-transparent grow focus:outline-none"
        />
      </div>
      {results}
    </DialogContent>
  )
}
