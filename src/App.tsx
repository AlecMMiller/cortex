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
import { HeadingNode, HeadingTagType, QuoteNode } from "@lexical/rich-text";
import { CodeNode } from "@lexical/code";
import { LinkNode } from "@lexical/link";
import { TableOfContentsEntry, TableOfContentsPlugin } from "@lexical/react/LexicalTableOfContentsPlugin";
import { NodeKey } from "lexical";
import { useLexicalComposerContext } from "@lexical/react/LexicalComposerContext";
import { useRef, useState } from "react";

const BASE_TEXT = 'text-text-normal font-prose'

const NORMAL_TEXT = `${BASE_TEXT} text-base lg:text-lg font-semibold`

const theme = {
  paragraph: `${NORMAL_TEXT} py-4 leading-loose'`,
  text: {
    bold: 'font-extrabold text-text-bold',
    strikethrough: 'line-through',
    code: 'bg-background-code p-1 rounded-md'
  },
  quote: `${NORMAL_TEXT} border-l-4 border-quote pl-4`,
  hr: 'border-0 bg-separator h-px',
  heading: {
    h1: 'text-text-normal font-prose font-semibold text-lg lg:text-5xl py-2 my-2 border-b border-separator',
    h2: 'text-text-normal font-prose font-semibold text-lg lg:text-4xl py-2 my-2 border-b border-separator',
    h3: 'text-text-normal font-prose font-semibold text-lg lg:text-3xl py-4',
    h4: 'text-text-normal font-prose font-semibold text-lg lg:text-2xl py-4',
    h5: 'text-text-normal font-prose font-semibold text-lg lg:text-xl py-4',
    h6: 'text-text-normal font-prose font-semibold text-lg lg:text-lg py-4',
  },
  code: 'text-lg lg:text-xl text-text-normal bg-background-code py-0 border-1 block rounded-md p-4',
  list: {
    nested: {
      listitem: ''
    },
    listitem: `${NORMAL_TEXT} list-disc ml-6`,
    codeHighlight: {
      builtin: 'text-text-bold',
    }
  }
}

function onError(error: any) {
  console.error(error);
}

function PageNavigator(props: { tableOfContents: Array<[key: NodeKey, text: string, tag: HeadingTagType]> }): JSX.Element {
  const [selectedKey, setSelectedKey] = useState('');
  const [editor] = useLexicalComposerContext();
  const selectedIndex = useRef(0);
  function scrollToNode(key: NodeKey, currIndex: number) {
    editor.getEditorState().read(() => {
      const domElement = editor.getElementByKey(key);
      if (domElement !== null) {
        domElement.scrollIntoView();
        setSelectedKey(key);
        selectedIndex.current = currIndex;
      }
    });
  }

  const entries = props.tableOfContents.map((entry, index) => {
    const key = entry[0]
    return <div key={entry[0]} onClick={() => scrollToNode(key, index)}
    role="button">
      {entry[1]}
    </div>
  })
  return <div className={`${NORMAL_TEXT} p-2 lg:p-9 gap-2 flex flex-col`}>
    {entries}
  </div>
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
      <div className="flex flex-col">
        <h1 className="text-text-normal font-semibold font-prose text-4xl pt-12 text-center">Example Title</h1>
        <RichTextPlugin
          contentEditable={<ContentEditable className="p-10 focus:outline-none max-w-4xl" />}
          placeholder={<div>Enter some text...</div>}
          ErrorBoundary={LexicalErrorBoundary}
        />
      </div>
      <MarkdownShortcutPlugin />
      <HistoryPlugin />
      <AutoFocusPlugin />
      <TableOfContentsPlugin>
        {(tableOfContentsArray) => {
          return <PageNavigator tableOfContents={tableOfContentsArray} />;
        }}
      </TableOfContentsPlugin>
    </LexicalComposer>
  );
}

function App() {
  return (
    <div className="bg-background-base-default text-3xl w-screen min-h-screen flex flex-row justify-between">
      <Editor />
    </div>
  );
}

export default App;
