import { commands } from '@/bindings'
import { newBuildQueryMethods } from './common'

export const {
  useType: useAvailableTagsContaining,
  buildPrefetchType: buildPretchAvailableTagsContaining,
} = newBuildQueryMethods(
  commands.getAvailableTagsContaining,
  (content: string, _maxResults, noteUuid: string) => [
    'note',
    noteUuid,
    'tags_containing',
    content,
  ],
)
