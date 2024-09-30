/**
 * Originally derived from LexicalClickableLinkPlugin
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 */

import { $isInternalLinkNode } from '@/nodes/InternalLink'
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext'
import { $findMatchingParent, isHTMLAnchorElement } from '@lexical/utils'
import { useNavigate, useRouter } from '@tanstack/react-router'
import {
  $getNearestNodeFromDOMNode,
  $getSelection,
  $isElementNode,
  $isRangeSelection,
  getNearestEditorFromDOMNode,
} from 'lexical'
import { useEffect } from 'react'

function findMatchingDOM<T extends Node>(
  startNode: Node,
  predicate: (node: Node) => node is T,
): T | null {
  let node: Node | null = startNode
  while (node != null) {
    if (predicate(node)) {
      return node
    }
    node = node.parentNode
  }
  return null
}

export function InternalLinkPlugin({
  newTab = true,
  disabled = false,
}: {
  newTab?: boolean
  disabled?: boolean
}): null {
  const [editor] = useLexicalComposerContext()
  const navigation = useNavigate({ from: '/' })
  console.log(navigation)
  //console.log(router)

  useEffect(() => {
    const onClick = (event: MouseEvent) => {
      const target = event.target
      if (!(target instanceof Node)) {
        return
      }
      const nearestEditor = getNearestEditorFromDOMNode(target)

      if (nearestEditor === null) {
        return
      }

      let url = null
      nearestEditor.update(() => {
        const clickedNode = $getNearestNodeFromDOMNode(target)
        if (clickedNode !== null) {
          const maybeLinkNode = $findMatchingParent(clickedNode, $isElementNode)
          if (!disabled) {
            if ($isInternalLinkNode(maybeLinkNode)) {
              url = maybeLinkNode.getURL()
            }
          }
        }
      })

      if (url === null || url === '') {
        return
      }

      // Allow user to select link text without follwing url
      const selection = editor.getEditorState().read($getSelection)
      if ($isRangeSelection(selection) && !selection.isCollapsed()) {
        event.preventDefault()
        return
      }

      navigation({ to: url })
      event.preventDefault()
    }

    const onMouseUp = (event: MouseEvent) => {
      if (event.button === 1) {
        onClick(event)
      }
    }

    return editor.registerRootListener((rootElement, prevRootElement) => {
      if (prevRootElement !== null) {
        prevRootElement.removeEventListener('click', onClick)
        prevRootElement.removeEventListener('mouseup', onMouseUp)
      }
      if (rootElement !== null) {
        rootElement.addEventListener('click', onClick)
        rootElement.addEventListener('mouseup', onMouseUp)
      }
    })
  }, [editor, newTab, disabled])

  return null
}
