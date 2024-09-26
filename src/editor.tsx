import { AutoFocusPlugin } from '@lexical/react/LexicalAutoFocusPlugin'
import {
  InitialConfigType,
  LexicalComposer,
} from '@lexical/react/LexicalComposer'
import { ContentEditable } from '@lexical/react/LexicalContentEditable'
import { LexicalErrorBoundary } from '@lexical/react/LexicalErrorBoundary'
import { HistoryPlugin } from '@lexical/react/LexicalHistoryPlugin'
import { ListPlugin } from '@lexical/react/LexicalListPlugin'
import { MarkdownShortcutPlugin } from '@lexical/react/LexicalMarkdownShortcutPlugin'
import { RichTextPlugin } from '@lexical/react/LexicalRichTextPlugin'
import {
  LexicalTypeaheadMenuPlugin,
  useBasicTypeaheadTriggerMatch,
} from '@lexical/react/LexicalTypeaheadMenuPlugin'
import { LexicalAutoLinkPlugin } from './utils/AutoLink'
import { ExternalLinkPlugin } from './utils/ExternalLink'
import { TableOfContentsPlugin } from '@lexical/react/LexicalTableOfContentsPlugin'
import PageNavigator from './elements/PageNavigator'
import { OnChangePlugin } from '@lexical/react/LexicalOnChangePlugin'
import { EDITOR_THEME } from './style'
import { ListItemNode, ListNode } from '@lexical/list'
import { HorizontalRuleNode } from '@lexical/react/LexicalHorizontalRuleNode'
import { HeadingNode, QuoteNode } from '@lexical/rich-text'
import { CodeNode } from '@lexical/code'
import { AutoLinkNode, LinkNode } from '@lexical/link'
import { EditorState, LexicalEditor } from 'lexical'
import { invoke } from '@tauri-apps/api/core'
import { NoteData } from './types'
import { renameNote } from './commands/note'
import * as Popover from '@radix-ui/react-popover'
import { MutableRefObject } from 'react'
import { createPortal } from 'react-dom'
import WikiLinkPlugin from './utils/WikiLink'

function onChange(uuid: string, state: EditorState): void {
  const json = state.toJSON()
  const serialized = JSON.stringify(json)
  invoke('editor_change_state', { uuid, body: serialized })
}

function onTitleChange(uuid: string, title: string): void {
  console.log('title changed', uuid, title)
  renameNote(uuid, title)
}

function onError(error: any): void {
  console.error(error)
}
interface EditorProps {
  readonly note: NoteData
}

export default function Editor(props: EditorProps): JSX.Element {
  const note = props.note

  const initialEditorState = note.body !== '' ? note.body : undefined

  const initialConfig: InitialConfigType = {
    namespace: 'MyEditor',
    editorState: initialEditorState,
    theme: EDITOR_THEME,
    onError,
    nodes: [
      ListNode,
      ListItemNode,
      HorizontalRuleNode,
      HeadingNode,
      QuoteNode,
      CodeNode,
      LinkNode,
      AutoLinkNode,
    ],
  }

  return (
    <LexicalComposer initialConfig={initialConfig}>
      <ListPlugin />
      <div className="flex flex-col overflow-y-auto flex-1 items-center">
        <h1
          onInput={(e) => {
            onTitleChange(note.uuid, e.currentTarget.innerText)
          }}
          contentEditable
          className="text-text font-semibold font-prose text-4xl mt-12 text-center"
        >
          {note.title}
        </h1>
        <RichTextPlugin
          contentEditable={
            <ContentEditable className="p-10 focus:outline-none max-w-4xl w-full" />
          }
          placeholder={<></>}
          ErrorBoundary={LexicalErrorBoundary}
        />
      </div>
      <MarkdownShortcutPlugin />
      <HistoryPlugin />
      <AutoFocusPlugin />
      <LexicalAutoLinkPlugin />
      <WikiLinkPlugin />
      <ExternalLinkPlugin />
      <TableOfContentsPlugin>
        {(tableOfContentsArray) => {
          return <PageNavigator tableOfContents={tableOfContentsArray} />
        }}
      </TableOfContentsPlugin>
      <OnChangePlugin
        onChange={(state: EditorState) => {
          onChange(note.uuid, state)
        }}
      />
    </LexicalComposer>
  )
}
