/*
 * Originally derived from LexicalClickableLinkPlugin
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 */

import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext'
import { $findMatchingParent } from '@lexical/utils'
import {
  $getNearestNodeFromDOMNode,
  $getSelection,
  $isElementNode,
  $isRangeSelection,
  getNearestEditorFromDOMNode,
  LexicalEditor,
  LexicalNode,
} from 'lexical'
import { useEffect } from 'react'

type CheckNodeFunction<T extends LexicalNode> = (
  node: LexicalNode | null,
) => node is T
type GetUrlFunction<T> = (node: T) => string

function getUrl<T extends LexicalNode>(
  disabled: boolean,
  event: MouseEvent,
  checker: CheckNodeFunction<T>,
  extractor: GetUrlFunction<T>,
): string | null {
  const target = event.target
  if (!(target instanceof Node)) {
    return null
  }
  const nearestEditor = getNearestEditorFromDOMNode(target)

  if (nearestEditor === null) {
    return null
  }

  let url = null
  nearestEditor.update(() => {
    const clickedNode = $getNearestNodeFromDOMNode(target)
    if (clickedNode !== null) {
      const maybeLinkNode = $findMatchingParent(clickedNode, $isElementNode)
      if (!disabled) {
        if (checker(maybeLinkNode)) {
          url = extractor(maybeLinkNode)
        }
      }
    }
  })

  return url === '' ? null : url
}

function registerOnClick(
  editor: LexicalEditor,
  onClick: (event: MouseEvent) => void,
) {
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
}

export function makeLinkPlugin<T extends LexicalNode>(
  checker: CheckNodeFunction<T>,
  extractor: GetUrlFunction<T>,
  action: (url: string) => void,
) {
  const GenericPlugin = ({
    newTab = true,
    disabled = false,
  }: {
    newTab?: boolean
    disabled?: boolean
  }): null => {
    const [editor] = useLexicalComposerContext()

    useEffect(() => {
      const onClick = (event: MouseEvent) => {
        const url = getUrl(disabled, event, checker, extractor)

        if (url === null) return

        // Allow user to select link text without follwing url
        const selection = editor.getEditorState().read($getSelection)
        if ($isRangeSelection(selection) && !selection.isCollapsed()) {
          event.preventDefault()
          return
        }
        action(url)
        event.preventDefault()
      }

      return registerOnClick(editor, onClick)
    }, [editor, newTab, disabled])

    return null
  }

  return GenericPlugin
}
