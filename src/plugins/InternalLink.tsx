import { $isInternalLinkNode, InternalLinkNode } from '@/nodes/InternalLink'
import { useNavigate } from '@tanstack/react-router'
import { makeLinkPlugin } from './LinkCommon'

export function InternalLinkPlugin() {
  const navigate = useNavigate()
  const Nested = makeLinkPlugin<InternalLinkNode>(
    $isInternalLinkNode,
    (node: InternalLinkNode) => {
      return node.getURL()
    },
    (url: string) => {
      navigate({ to: url })
    },
  )
  return <Nested />
}
