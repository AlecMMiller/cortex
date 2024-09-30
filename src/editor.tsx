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
import { LexicalAutoLinkPlugin } from './plugins/AutoLink'
import { ExternalLinkPlugin } from './plugins/ExternalLink'
import { InternalLinkPlugin } from './plugins/InternalLink'
import { TableOfContentsPlugin } from '@lexical/react/LexicalTableOfContentsPlugin'
import PageNavigator from './elements/PageNavigator'
import { OnChangePlugin } from '@lexical/react/LexicalOnChangePlugin'
import { EDITOR_THEME } from './style'
import { ListItemNode, ListNode } from '@lexical/list'
import { HorizontalRuleNode } from '@lexical/react/LexicalHorizontalRuleNode'
import { HeadingNode, QuoteNode } from '@lexical/rich-text'
import { CodeNode } from '@lexical/code'
import { AutoLinkNode, LinkNode } from '@lexical/link'
import { EditorState, TextNode } from 'lexical'
import { invoke } from '@tauri-apps/api/core'
import { NoteData } from './types'
import { makeNoteQueryKey, renameNote } from './commands/note'
import WikiLinkPlugin from './plugins/WikiLink'
import { InternalLinkNode } from './nodes/InternalLink'
import { useQueryClient, QueryClient } from '@tanstack/react-query'

function onChange(
  queryClient: QueryClient,
  uuid: string,
  state: EditorState,
): void {
  const queryKey = makeNoteQueryKey(uuid)
  const json = state.toJSON()
  const serialized = JSON.stringify(json)
  invoke('update_note', { uuid, body: serialized })
  queryClient.invalidateQueries({ queryKey })
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
  const queryClient = useQueryClient()
  const note = props.note

  const initialEditorState = note.body !== '' ? note.body : undefined

  const initialConfig: InitialConfigType = {
    namespace: 'MyEditor',
    editorState: initialEditorState,
    theme: EDITOR_THEME,
    onError,
    nodes: [
      TextNode,
      ListNode,
      ListItemNode,
      HorizontalRuleNode,
      HeadingNode,
      QuoteNode,
      CodeNode,
      LinkNode,
      AutoLinkNode,
      InternalLinkNode,
    ],
  }

  return (
    <LexicalComposer initialConfig={initialConfig} key={note.uuid}>
      <ListPlugin />
      <div className="flex flex-col overflow-y-auto flex-1 items-center">
        <h1
          onInput={(e) => {
            onTitleChange(note.uuid, e.currentTarget.innerText)
          }}
          suppressContentEditableWarning
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
      <InternalLinkPlugin />
      <OnChangePlugin
        onChange={(state: EditorState) => {
          onChange(queryClient, note.uuid, state)
        }}
      />
    </LexicalComposer>
  )
}
