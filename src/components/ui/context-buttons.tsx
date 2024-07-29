import { ClipboardPaste, Copy, LucideIcon, Scissors } from 'lucide-react';
import { ContextMenuItem } from './context-menu';
import { $getHtmlContent } from '@lexical/clipboard'
import { writeHtml, writeText } from '@tauri-apps/plugin-clipboard-manager';
import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import { $getSelection } from 'lexical';

function ButtonWithIcon(props: { icon: LucideIcon, text: string, onClick: () => void }): JSX.Element {
    let Icon = props.icon
    let iconActual = <Icon size={18} className='text-text-soft' />
    return (
        <ContextMenuItem onClick={props.onClick}>{iconActual}{props.text}</ContextMenuItem>
    )
}

export function CopyButton(): JSX.Element {
    const [editor] = useLexicalComposerContext();
    return (
        <ButtonWithIcon onClick={() => {
            console.log(editor)
            editor.update(() => {
                const selection = $getSelection()
                const plainText = selection?.getTextContent()
                const html = $getHtmlContent(editor)
                writeHtml(html, plainText)
                if (plainText) writeText(plainText)
            })
        }} icon={Copy} text="Copy" />
    )
}

export function CutButton(): JSX.Element {
    return (
        <ButtonWithIcon onClick={() => document.execCommand('copy')} icon={Scissors} text="Cut" />
    )
}

export function PasteButton(): JSX.Element {
    return (
        <ButtonWithIcon onClick={() => document.execCommand('copy')} icon={ClipboardPaste} text="Paste" />
    )
}

export function CutCopyPaste(): JSX.Element {
    return (
        <>
            <CutButton />
            <CopyButton />
            <PasteButton />
        </>
    )
}