import { buildQueryMethods } from './common'

type TagSearch = {
  noteUuid: string
  content: string
  maxResults: number
}

interface Tag {
  uuid: string
  title: string
}

export const {
  useType: useAvailableTagsContaining,
  buildPrefetchType: buildPretchAvailableTagsContaining,
} = buildQueryMethods<TagSearch, [Tag[], boolean]>({
  command: 'get_available_tags_containing',
  makeKey: (data: TagSearch) => {
    return ['tags', 'containing', data.content]
  },
})
