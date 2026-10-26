import { useLayoutEffect, useRef } from 'react';
import { Compartment, EditorState } from '@codemirror/state';
import { EditorView, lineNumbers, highlightActiveLine, keymap, drawSelection } from '@codemirror/view';
import { defaultKeymap, history, historyKeymap, indentWithTab } from '@codemirror/commands';
import { sql, PLSQL } from '@codemirror/lang-sql';
import { syntaxHighlighting, defaultHighlightStyle } from '@codemirror/language';
import { oneDark } from '@codemirror/theme-one-dark';

interface Props {
  id: number; text: string; dark: boolean;
  states: Map<number, EditorState>;
  onChange: (text: string) => void;
  onView: (view: EditorView | null) => void;
}
const theme = new Compartment();
export function Editor({ id, text, dark, states, onChange, onView }: Props) {
  const host = useRef<HTMLDivElement>(null);
  const view = useRef<EditorView | null>(null);
  const callbacks = useRef({ onChange, onView });
  callbacks.current = { onChange, onView };
  useLayoutEffect(() => {
    if (!host.current) return;
    const state = states.get(id) ?? EditorState.create({ doc: text, extensions: [
      lineNumbers(), history(), drawSelection(), highlightActiveLine(),
      sql({ dialect: PLSQL }), syntaxHighlighting(defaultHighlightStyle, { fallback: true }),
      keymap.of([...defaultKeymap, ...historyKeymap, indentWithTab]),
      theme.of(dark ? oneDark : []),
      EditorView.contentAttributes.of({ 'aria-label': 'SQL editor', spellcheck: 'false' }),
      EditorView.updateListener.of(update => {
        if (update.docChanged) callbacks.current.onChange(update.state.doc.toString());
      }),
    ] });
    const editor = new EditorView({ state, parent: host.current });
    view.current = editor;
    callbacks.current.onView(editor);
    editor.focus();
    return () => {
      states.set(id, editor.state);
      callbacks.current.onView(null);
      editor.destroy();
      view.current = null;
    };
    // Per-tab states preserve selection, undo and scroll. Content updates originate in this view.
  }, [id, states]);
  useLayoutEffect(() => { view.current?.dispatch({ effects: theme.reconfigure(dark ? oneDark : []) }); }, [dark, id]);
  return <div className="editor" data-region="editor" ref={host} />;
}
