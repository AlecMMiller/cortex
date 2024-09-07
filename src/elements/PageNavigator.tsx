import { useLexicalComposerContext } from "@lexical/react/LexicalComposerContext";
import { HeadingTagType } from "@lexical/rich-text";
import { NodeKey } from "lexical";
import { useRef } from "react";
import { BASE_TEXT } from "../style";

export default function PageNavigator(props: { tableOfContents: Array<[key: NodeKey, text: string, tag: HeadingTagType]> }): JSX.Element {
    const [editor] = useLexicalComposerContext();
    const selectedIndex = useRef(0);
    function scrollToNode(key: NodeKey, currIndex: number) {
        editor.getEditorState().read(() => {
            const domElement = editor.getElementByKey(key);
            if (domElement !== null) {
                domElement.scrollIntoView();
                selectedIndex.current = currIndex;
            }
        });
    }

    const entries = props.tableOfContents.map((entry, index) => {
        const key = entry[0]
        return <div key={entry[0]} onClick={() => scrollToNode(key, index)}
            role="button"
            className={`hover:text-blue`}
        >
            {entry[1]}
        </div>
    })
    return <div className={`text-overlay0 p-2 lg:p-9 gap-2 flex flex-col text-base font-semibold min-w-56`}>
        {entries}
    </div>
}
