import { commands } from '@/bindings'
import { buildQueryMethods } from './common'

export const {
  useType: useAvailableTagsContaining,
  prefetchType: pretchAvailableTagsContaining,
} = buildQueryMethods(
  commands.getAvailableTagsContaining,
  (content: string, _maxResults, noteUuid: string) => [
    'note',
    noteUuid,
    'tags_containing',
    content,
  ],
)
