import { buildQueryMethods } from './common'

type TagSearch = {
  content: string
  maxResults: number
}

interface Tag {
  uuid: string
  title: string
}

export const {
  useType: useTagsContaining,
  buildPrefetchType: buildPretchTagsContaining,
} = buildQueryMethods<TagSearch, Tag[]>({
  command: 'get_tags_containing',
  makeKey: (data: TagSearch) => {
    return ['tags', 'containing', data.content]
  },
})
