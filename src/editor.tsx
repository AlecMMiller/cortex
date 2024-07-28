import { AutoFocusPlugin } from "@lexical/react/LexicalAutoFocusPlugin";
import { InitialConfigType, LexicalComposer } from "@lexical/react/LexicalComposer";
import { ContentEditable } from "@lexical/react/LexicalContentEditable";
import LexicalErrorBoundary from "@lexical/react/LexicalErrorBoundary";
import { HistoryPlugin } from "@lexical/react/LexicalHistoryPlugin";
import { ListPlugin } from "@lexical/react/LexicalListPlugin";
import { MarkdownShortcutPlugin } from "@lexical/react/LexicalMarkdownShortcutPlugin";
import { RichTextPlugin } from "@lexical/react/LexicalRichTextPlugin";
import LexicalAutoLinkPlugin from "./utils/AutoLink";
import { TableOfContentsPlugin } from "@lexical/react/LexicalTableOfContentsPlugin";
import PageNavigator from "./elements/PageNavigator";
import { OnChangePlugin } from "@lexical/react/LexicalOnChangePlugin";
import { EDITOR_THEME } from "./style";
import { ListItemNode, ListNode } from "@lexical/list";
import { HorizontalRuleNode } from "@lexical/react/LexicalHorizontalRuleNode";
import { HeadingNode, QuoteNode } from "@lexical/rich-text";
import { CodeNode } from "@lexical/code";
import { AutoLinkNode, LinkNode } from "@lexical/link";
import { EditorState } from "lexical";
import { invoke } from "@tauri-apps/api/core";
import { NoteData } from "./types";
import { renameNote } from "./commands/note";

function onChange(state: EditorState) {
    const json = state.toJSON()
    const serialized = JSON.stringify(json)
    invoke('editor_change_state', { state: serialized })
}

function onTitleChange(uuid: string, title: string) {
    console.log('title changed', uuid, title)
    renameNote(uuid, title)
}

function onError(error: any) {
    console.error(error);
}


const initialConfig: InitialConfigType = {
    namespace: 'MyEditor',
    theme: EDITOR_THEME,
    onError,
    nodes: [ListNode, ListItemNode, HorizontalRuleNode, HeadingNode, QuoteNode, CodeNode, LinkNode, AutoLinkNode]
};

interface EditorProps {
    note: NoteData
}

export default function Editor(props: EditorProps): JSX.Element {
    const note = props.note
    return (
        <LexicalComposer initialConfig={initialConfig}>
            <ListPlugin />
            <div className="flex flex-col overflow-y-auto h-full grow items-center">
                <h1 onInput={(e) => {onTitleChange(note.uuid, e.currentTarget.innerText)}}contentEditable className="text-text-normal font-semibold font-prose text-4xl mt-12 text-center">{note.title}</h1>
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