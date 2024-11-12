import { commands } from '@/bindings'
import { buildQueryMethods } from './common'

export const {
  useType: useAvailableTagsContaining,
  buildPrefetchType: buildPretchAvailableTagsContaining,
} = buildQueryMethods(
  commands.getAvailableTagsContaining,
  (content: string, _maxResults, noteUuid: string) => [
    'note',
    noteUuid,
    'tags_containing',
    content,
  ],
)
