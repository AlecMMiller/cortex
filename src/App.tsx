import { InitialConfigType, LexicalComposer } from "@lexical/react/LexicalComposer";
import { RichTextPlugin } from "@lexical/react/LexicalRichTextPlugin";
import { ContentEditable } from "@lexical/react/LexicalContentEditable";
import { HistoryPlugin } from "@lexical/react/LexicalHistoryPlugin";
import { AutoFocusPlugin } from "@lexical/react/LexicalAutoFocusPlugin";
import LexicalErrorBoundary from "@lexical/react/LexicalErrorBoundary";
import { ListItemNode, ListNode } from "@lexical/list";
import { ListPlugin } from "@lexical/react/LexicalListPlugin";
import { MarkdownShortcutPlugin } from "@lexical/react/LexicalMarkdownShortcutPlugin";
import { HorizontalRuleNode } from "@lexical/react/LexicalHorizontalRuleNode";
import { HeadingNode, QuoteNode } from "@lexical/rich-text";
import { CodeNode } from "@lexical/code";
import { AutoLinkNode, LinkNode } from "@lexical/link";
import { TableOfContentsPlugin } from "@lexical/react/LexicalTableOfContentsPlugin";
import PageNavigator from "./elements/PageNavigator";
import { EDITOR_THEME } from "./style";
import LexicalAutoLinkPlugin from "./utils/AutoLink";
import { OnChangePlugin } from "@lexical/react/LexicalOnChangePlugin";
import { EditorState } from "lexical";
import { invoke } from '@tauri-apps/api/core'
import { useEffect } from "react";
import { getLastUpdated } from "./commands/note";

function onError(error: any) {
  console.error(error);
}

function onChange(state: EditorState) {
  const json = state.toJSON()
  console.log(json)
  const serialized = JSON.stringify(json)
  invoke('editor_change_state', { state: serialized })
}

function Editor() {
  console.log("Editor component loaded")
  useEffect(() => {
    async function loadLatest() {
      const result = await getLastUpdated()
      console.log("Got result")
      console.log(result)
    }
    loadLatest()
  }, [])
  const initialConfig: InitialConfigType = {
    namespace: 'MyEditor',
    theme: EDITOR_THEME,
    onError,
    nodes: [ListNode, ListItemNode, HorizontalRuleNode, HeadingNode, QuoteNode, CodeNode, LinkNode, AutoLinkNode]
  };

  return (
    <LexicalComposer initialConfig={initialConfig}>
      <ListPlugin />
      <div className="flex flex-col overflow-y-auto h-full grow items-center">
        <h1 contentEditable className="text-text-normal font-semibold font-prose text-4xl mt-12 text-center">Example Title</h1>
        <RichTextPlugin
          contentEditable={<ContentEditable className="p-10 focus:outline-none max-w-4xl w-full" />}
          placeholder={<></>}
          ErrorBoundary={LexicalErrorBoundary}
        />
      </div>
      <MarkdownShortcutPlugin />
      <HistoryPlugin />
      <AutoFocusPlugin />
      <LexicalAutoLinkPlugin />
      <TableOfContentsPlugin>
        {(tableOfContentsArray) => {
          return <PageNavigator tableOfContents={tableOfContentsArray} />;
        }}
      </TableOfContentsPlugin>
      <OnChangePlugin onChange={onChange} />
    </LexicalComposer>
  );
}

function App() {
  return (
    <div className="bg-background-base-default text-3xl w-screen h-screen flex flex-row justify-between">
      <Editor />
    </div>
  );
}

export default App;
