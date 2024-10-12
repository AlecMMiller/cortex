import { useTranslation } from 'react-i18next'
import { DialogContent } from '../ui/dialog'
import { DialogFunctionProps } from '../ui/nav-button'
import { Search } from 'lucide-react'
import { useEffect, useState } from 'react'
import { useSearchNotesByContent } from '@/commands/note'
import { NoteTitle } from '@/types'
import { NoteLink } from '../ui/note-link'

interface SearchResultProps {
  note: NoteTitle
  setOpen: (open: boolean) => void
}

function SearchResult(props: SearchResultProps): JSX.Element {
  return (
    <NoteLink
      onClick={() => {
        props.setOpen(false)
      }}
      note={props.note}
    />
  )
}

export function SearchDialog(props: DialogFunctionProps) {
  const [queryPhrase, setQueryPhrase] = useState('')
  const [cachedResults, setCachedResults] = useState<NoteTitle[]>([])
  const { t } = useTranslation()

  const { data } = useSearchNotesByContent(
    {
      maxResults: 10,
      content: queryPhrase,
    },
    {},
  )

  useEffect(() => {
    setCachedResults(data ?? [])
  }, [data])

  const results = cachedResults.map((result) => (
    <SearchResult setOpen={props.setOpen} key={result.uuid} note={result} />
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
