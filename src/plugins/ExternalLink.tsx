import { $isLinkNode, LinkNode } from '@lexical/link'
import { open } from '@tauri-apps/plugin-shell'
import { makeLinkPlugin } from './LinkCommon'

export const ExternalLinkPlugin = makeLinkPlugin<LinkNode>(
  $isLinkNode,
  (node: LinkNode) => {
    return node.sanitizeUrl(node.getURL())
  },
  (url: string) => {
    open(url)
  },
)
