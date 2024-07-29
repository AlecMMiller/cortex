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
import { ContextMenu, ContextMenuContent, ContextMenuTrigger } from "./components/ui/context-menu";
import { CutCopyPaste } from "./components/ui/context-buttons";

function onChange(uuid: string, state: EditorState) {
    const json = state.toJSON()
    const serialized = JSON.stringify(json)
    invoke('editor_change_state', { uuid, body: serialized })
}

function onTitleChange(uuid: string, title: string) {
    console.log('title changed', uuid, title)
    renameNote(uuid, title)
}

function onError(error: any) {
    console.error(error);
}
interface EditorProps {
    readonly note: NoteData
}

export default function Editor(props: EditorProps): JSX.Element {
    const note = props.note

    let initialEditorState = note.body !== "" ? note.body : undefined;

    const initialConfig: InitialConfigType = {
        namespace: 'MyEditor',
        editorState: initialEditorState,
        theme: EDITOR_THEME,
        onError,
        nodes: [ListNode, ListItemNode, HorizontalRuleNode, HeadingNode, QuoteNode, CodeNode, LinkNode, AutoLinkNode]
    };

    return (
        <LexicalComposer initialConfig={initialConfig}>
            <ListPlugin />
            <div className="flex flex-col overflow-y-auto h-full grow items-center">
                <h1 onInput={(e) => { onTitleChange(note.uuid, e.currentTarget.innerText) }} suppressContentEditableWarning contentEditable className="text-text-normal font-semibold font-prose text-4xl mt-12 text-center">{note.title}</h1>
                <ContextMenu>
                    <ContextMenuTrigger>
                        <RichTextPlugin
                            contentEditable={<ContentEditable className="p-10 focus:outline-none max-w-4xl w-full" />}
                            placeholder={<></>}
                            ErrorBoundary={LexicalErrorBoundary}
                        />
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <CutCopyPaste />
                    </ContextMenuContent>
                </ContextMenu>
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
            <OnChangePlugin onChange={(state: EditorState) => { onChange(note.uuid, state) }} />
        </LexicalComposer>
    );
}