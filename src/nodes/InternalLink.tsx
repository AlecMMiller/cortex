/**
 * Originally derived from lexical-link
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 */

import type {
  BaseSelection,
  DOMConversionMap,
  DOMConversionOutput,
  EditorConfig,
  LexicalNode,
  NodeKey,
  RangeSelection,
  SerializedElementNode,
} from 'lexical'

import { addClassNamesToElement, isHTMLAnchorElement } from '@lexical/utils'
import {
  $applyNodeReplacement,
  $isRangeSelection,
  ElementNode,
  Spread,
  TextNode,
} from 'lexical'

export type InternalLinkAttributes = {
  uuid: string
}

export type SerializedInternalLinkNode = Spread<
  InternalLinkAttributes,
  SerializedElementNode
>

type InternalLinkHTMLElementType = HTMLAnchorElement | HTMLSpanElement

/** @noInheritDoc */
export class InternalLinkNode extends ElementNode {
  /** @internal */
  __uuid: string

  static getType(): string {
    return 'internal-link'
  }

  static clone(node: InternalLinkNode): InternalLinkNode {
    return new InternalLinkNode(node.__uuid, node.__key)
  }

  constructor(uuid: string, key?: NodeKey) {
    super(key)
    this.__uuid = uuid
  }

  createDOM(config: EditorConfig): InternalLinkHTMLElementType {
    const element = document.createElement('a')
    element.href = this.getURL()
    //element.textContent = this.__title
    element.target = '_self'
    addClassNamesToElement(element, config.theme['internal-link'])
    return element
  }

  updateDOM(
    prevNode: InternalLinkNode,
    anchor: InternalLinkHTMLElementType,
    _config: EditorConfig,
  ): boolean {
    if (anchor instanceof HTMLAnchorElement) {
      const url = this.getURL()
      const uuid = this.__uuid

      if (uuid !== prevNode.__uuid) {
        anchor.href = url
      }
    }
    return false
  }

  static importDOM(): DOMConversionMap | null {
    return {
      a: (_node: Node) => ({
        conversion: $convertAnchorElement,
        priority: 1,
      }),
    }
  }

  static importJSON(
    serializedNode: SerializedInternalLinkNode,
  ): InternalLinkNode {
    const node = $createInternalLinkNode_(serializedNode.uuid)
    node.setFormat(serializedNode.format)
    node.setIndent(serializedNode.indent)
    node.setDirection(serializedNode.direction)
    return node
  }

  exportJSON(): SerializedInternalLinkNode {
    return {
      ...super.exportJSON(),
      uuid: this.getUuid(),
      type: 'internal-link',
      version: 1,
    }
  }

  getURL(): string {
    const uuid = this.getUuid()
    return `/notes/${uuid}`
  }

  getUuid(): string {
    return this.getLatest().__uuid
  }

  setUuid(uuid: string): void {
    const writeable = this.getWritable()
    writeable.__uuid = uuid
  }

  insertNewAfter(
    _: RangeSelection,
    restoreSelection = true,
  ): null | ElementNode {
    const linkNode = $createInternalLinkNode_(this.__uuid)
    this.insertAfter(linkNode, restoreSelection)
    return linkNode
  }

  canInsertTextBefore(): false {
    return false
  }

  canInsertTextAfter(): false {
    return false
  }

  canBeEmpty(): false {
    return false
  }

  isInline(): true {
    return true
  }

  extractWithChild(
    _child: LexicalNode,
    selection: BaseSelection,
    _destination: 'clone' | 'html',
  ): boolean {
    if (!$isRangeSelection(selection)) {
      return false
    }

    const anchorNode = selection.anchor.getNode()
    const focusNode = selection.focus.getNode()

    return (
      this.isParentOf(anchorNode) &&
      this.isParentOf(focusNode) &&
      selection.getTextContent().length > 0
    )
  }
}

function $convertAnchorElement(domNode: Node): DOMConversionOutput {
  let node = null
  if (isHTMLAnchorElement(domNode)) {
    const content = domNode.textContent
    if ((content !== null && content !== '') || domNode.children.length > 0) {
      const href = domNode.getAttribute('href')
      const uuid = href?.split('/')[1]
      if (uuid !== undefined) {
        node = $createInternalLinkNode_(uuid)
      }
    }
  }
  return { node }
}

function $createInternalLinkNode_(uuid: string): InternalLinkNode {
  return $applyNodeReplacement(new InternalLinkNode(uuid))
}

/**
 * Takes a URL and creates an InternalLinkNode.
 * @param uuid - The UUID of the note this links to
 * @param title - Name to display on the link
 * @returns The LinkNode.
 */
export function $createInternalLinkNode(
  uuid: string,
  title: string,
): InternalLinkNode {
  const child = new TextNode(title)
  const node = new InternalLinkNode(uuid)
  node.append(child)
  return $applyNodeReplacement(node)
}

/**
 * Determines if node is an InternalLinkNode.
 * @param node - The node to be checked.
 * @returns true if node is an InternalLinkNode, false otherwise.
 */
export function $isInternalLinkNode(
  node: LexicalNode | null | undefined,
): node is InternalLinkNode {
  return node instanceof InternalLinkNode
}
