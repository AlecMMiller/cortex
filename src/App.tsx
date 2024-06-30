import { LexicalComposer } from "@lexical/react/LexicalComposer";
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
import { LinkNode } from "@lexical/link";

const BASE_TEXT = 'text-text-normal font-prose'

const NORMAL_TEXT = `${BASE_TEXT} text-base lg:text-lg font-semibold`

const theme = {
  paragraph: `${NORMAL_TEXT} py-4 leading-loose'`,
  text: {
    bold: 'font-extrabold text-text-bold',
    strikethrough: 'line-through'
  },
  heading: {
    h1: 'text-text-normal font-prose font-semibold text-lg lg:text-5xl py-4',
    h2: 'text-text-normal font-prose font-semibold text-lg lg:text-4xl py-4',
    h3: 'text-text-normal font-prose font-semibold text-lg lg:text-3xl py-4',
    h4: 'text-text-normal font-prose font-semibold text-lg lg:text-2xl py-4',
    h5: 'text-text-normal font-prose font-semibold text-lg lg:text-xl py-4',
    h6: 'text-text-normal font-prose font-semibold text-lg lg:text-lg py-4',
  },
  list: {
    nested: {
      listitem: ''
    },
    listitem: `${NORMAL_TEXT} list-disc ml-6`
  }
}

function onError(error: any) {
  console.error(error);
}

function Editor() {
  const initialConfig = {
    namespace: 'MyEditor',
    theme,
    onError,
    nodes: [ListNode, ListItemNode, HorizontalRuleNode, HeadingNode, QuoteNode, CodeNode, LinkNode]
  };

  return (
    <LexicalComposer initialConfig={initialConfig}>
      <ListPlugin />
      <RichTextPlugin
        contentEditable={<ContentEditable className="p-10 focus:outline-none max-w-4xl" />}
        placeholder={<div>Enter some text...</div>}
        ErrorBoundary={LexicalErrorBoundary}
      />
      <MarkdownShortcutPlugin />
      <HistoryPlugin />
      <AutoFocusPlugin />
    </LexicalComposer>
  );
}

function App() {
  return (
    <div className="bg-background-base-default text-3xl w-screen min-h-screen flex flex-row justify-between">
      <div className="flex-grow"/>
      <div className="flex flex-col">
        <h1 className="text-text-normal font-semibold font-prose text-4xl pt-12 text-center">Example Title</h1>
      <Editor />
      </div>
      <div className="flex-grow"/>
    </div>
  );
}

export default App;
