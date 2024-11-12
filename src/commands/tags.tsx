import { commands } from '@/bindings'
import { newBuildQueryMethods } from './common'

export const {
  useType: useAvailableTagsContaining,
  buildPrefetchType: buildPretchAvailableTagsContaining,
} = newBuildQueryMethods(
  commands.getAvailableTagsContaining,
  (content: string, _max_results, note_uuid: string) => [
    'note',
    note_uuid,
    'tags_containing',
    content,
  ],
)
