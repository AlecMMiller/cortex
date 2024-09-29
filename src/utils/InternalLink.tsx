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
  LexicalCommand,
  LexicalNode,
  NodeKey,
  RangeSelection,
  SerializedElementNode,
} from 'lexical'

import {
  $findMatchingParent,
  addClassNamesToElement,
  isHTMLAnchorElement,
} from '@lexical/utils'
import {
  $applyNodeReplacement,
  $getSelection,
  $isElementNode,
  $isRangeSelection,
  createCommand,
  ElementNode,
  Spread,
} from 'lexical'

export type InternalLinkAttributes = {
  uuid: string
  title: string
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
  /** @internal */
  __title: string

  static getType(): string {
    return 'internal-link'
  }

  static clone(node: InternalLinkNode): InternalLinkNode {
    return new InternalLinkNode(node.__uuid, node.__title, node.__key)
  }

  constructor(uuid: string, title: string, key?: NodeKey) {
    super(key)
    this.__title = title
    this.__uuid = uuid
  }

  createDOM(config: EditorConfig): InternalLinkHTMLElementType {
    const element = document.createElement('a')
    element.href = this.getURL()
    element.title = this.__title
    element.textContent = this.__title
    addClassNamesToElement(element, config.theme.link)
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
      const title = this.__title

      if (uuid !== prevNode.__uuid) {
        anchor.href = url
      }

      if (title !== prevNode.__title) {
        anchor.title = title
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
    const node = $createInternalLinkNode(
      serializedNode.uuid,
      serializedNode.title,
    )
    node.setFormat(serializedNode.format)
    node.setIndent(serializedNode.indent)
    node.setDirection(serializedNode.direction)
    return node
  }

  exportJSON(): SerializedInternalLinkNode {
    return {
      ...super.exportJSON(),
      uuid: this.getUuid(),
      title: this.getTitle(),
      type: 'internal-link',
      version: 1,
    }
  }

  getURL(): string {
    const uuid = this.getUuid()
    return `notes/${uuid}`
  }

  getUuid(): string {
    return this.getLatest().__uuid
  }

  setUuid(uuid: string): void {
    const writeable = this.getWritable()
    writeable.__uuid = uuid
  }

  getTitle(): string {
    return this.getLatest().__title
  }

  setTitle(title: string): void {
    const writable = this.getWritable()
    writable.__title = title
  }

  insertNewAfter(
    _: RangeSelection,
    restoreSelection = true,
  ): null | ElementNode {
    const linkNode = $createInternalLinkNode(this.__uuid, this.__title)
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
      const title = domNode.getAttribute('title')
      const href = domNode.getAttribute('href')
      const uuid = href?.split('/')[1]
      if (title !== null && uuid !== undefined) {
        node = $createInternalLinkNode(uuid, title)
      }
    }
  }
  return { node }
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
  return $applyNodeReplacement(new InternalLinkNode(uuid, title))
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

export const TOGGLE_LINK_COMMAND: LexicalCommand<
  string | ({ url: string } & LinkAttributes) | null
> = createCommand('TOGGLE_LINK_COMMAND')

/**
 * Generates or updates a LinkNode. It can also delete a LinkNode if the URL is null,
 * but saves any children and brings them up to the parent node.
 * @param url - The URL the link directs to.
 * @param attributes - Optional HTML a tag attributes. \\{ target, rel, title \\}
 */
export function $toggleLink(
  url: null | string,
  attributes: LinkAttributes = {},
): void {
  const { target, title } = attributes
  const rel = attributes.rel === undefined ? 'noreferrer' : attributes.rel
  const selection = $getSelection()

  if (!$isRangeSelection(selection)) {
    return
  }
  const nodes = selection.extract()

  if (url === null) {
    // Remove LinkNodes
    nodes.forEach((node) => {
      const parentLink = $findMatchingParent(
        node,
        (parent): parent is LinkNode => !$isLinkNode(parent),
      )

      if (parentLink) {
        const children = parentLink.getChildren()

        for (let i = 0; i < children.length; i++) {
          parentLink.insertBefore(children[i])
        }

        parentLink.remove()
      }
    })
  } else {
    // Add or merge LinkNodes
    if (nodes.length === 1) {
      const firstNode = nodes[0]
      // if the first node is a LinkNode or if its
      // parent is a LinkNode, we update the URL, target and rel.
      const linkNode = $getAncestor(firstNode, $isLinkNode)
      if (linkNode !== null) {
        linkNode.setURL(url)
        if (target !== undefined) {
          linkNode.setTarget(target)
        }
        if (rel !== null) {
          linkNode.setRel(rel)
        }
        if (title !== undefined) {
          linkNode.setTitle(title)
        }
        return
      }
    }

    let prevParent: ElementNode | LinkNode | null = null
    let linkNode: LinkNode | null = null

    nodes.forEach((node) => {
      const parent = node.getParent()

      if (
        parent === linkNode ||
        parent === null ||
        ($isElementNode(node) && !node.isInline())
      ) {
        return
      }

      if ($isLinkNode(parent)) {
        linkNode = parent
        parent.setURL(url)
        if (target !== undefined) {
          parent.setTarget(target)
        }
        if (rel !== null) {
          linkNode.setRel(rel)
        }
        if (title !== undefined) {
          linkNode.setTitle(title)
        }
        return
      }

      if (!parent.is(prevParent)) {
        prevParent = parent
        linkNode = $createLinkNode(url, { rel, target, title })

        if ($isLinkNode(parent)) {
          if (node.getPreviousSibling() === null) {
            parent.insertBefore(linkNode)
          } else {
            parent.insertAfter(linkNode)
          }
        } else {
          node.insertBefore(linkNode)
        }
      }

      if ($isLinkNode(node)) {
        if (node.is(linkNode)) {
          return
        }
        if (linkNode !== null) {
          const children = node.getChildren()

          for (let i = 0; i < children.length; i++) {
            linkNode.append(children[i])
          }
        }

        node.remove()
        return
      }

      if (linkNode !== null) {
        linkNode.append(node)
      }
    })
  }
}

function $getAncestor<NodeType extends LexicalNode = LexicalNode>(
  node: LexicalNode,
  predicate: (ancestor: LexicalNode) => ancestor is NodeType,
) {
  let parent = node
  while (parent !== null && parent.getParent() !== null && !predicate(parent)) {
    parent = parent.getParentOrThrow()
  }
  return predicate(parent) ? parent : null
}
