import { useEffect, useLayoutEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { EditorView } from '@codemirror/view';
import type { EditorState } from '@codemirror/state';
import { changeTab, contract, sqlText, type Selection } from './model';
import { Editor } from './Editor';
import { Explorer } from './Explorer';
import { Grid } from './Grid';
import { Splitter } from './Splitter';
import { report, startFrames, wallNs, type Config } from './diagnostics';

export function App({ config }: { config: Config }) {
  const [tabs, setTabs] = useState(() => Array.from({ length: contract.tabs }, (_, id) => ({ id: id + 1, text: sqlText(config.largeSql) })));
  const nextId = useRef(contract.tabs + 1);
  const [active, setActive] = useState(0);
  const [dark, setDark] = useState(true);
  const [sidebar, setSidebar] = useState(220);
  const [editorHeight, setEditorHeight] = useState(200);
  const [height, setHeight] = useState(window.innerHeight);
  const [selection, setSelection] = useState<Selection>([0, 0]);
  const [revision, setRevision] = useState(0);
  const [scrollRow, setScrollRow] = useState(0);
  const [run, setRun] = useState('idle');
  const [outcome, setOutcome] = useState('success');
  const outcomeRef = useRef('success');
  const [error, setError] = useState('');
  const [focus, setFocus] = useState('Editor');
  const [elapsed, setElapsed] = useState(0);
  const states = useRef(new Map<number, EditorState>());
  const view = useRef<EditorView | null>(null);
  const inFlight = useRef(false);
  const tab = tabs[active];
  const visibleRows = run === 'empty' ? 0 : config.rows;

  const openTab = () => {
    const id = nextId.current++;
    setTabs(t => [...t, { id, text: '' }]); setActive(tabs.length);
  };
  const closeTab = () => {
    if (tabs.length === 1) return;
    states.current.delete(tab.id);
    setTabs(t => t.filter((_, i) => i !== active)); setActive(Math.min(active, tabs.length - 2));
  };
  const runQuery = async () => {
    if (inFlight.current) return;
    inFlight.current = true; setRun('running'); setError('');
    const start = performance.now();
    try {
      const result = await invoke<string>('run_query', { outcome: outcomeRef.current });
      setRun(result);
      if (result === 'done' || result === 'empty') { setSelection([0, 0]); setRevision(v => v + 1); }
    } catch (reason) { setRun('error'); setError(String(reason)); }
    finally { inFlight.current = false; setElapsed(performance.now() - start); }
  };
  const cancel = () => {
    if (!inFlight.current) return;
    setRun('cancelling');
    void invoke('cancel_query').catch(() => { setError('Cancellation request failed. Wait for mock completion.'); });
  };
  const focusPane = (pane: string) => {
    if (pane === 'Editor') view.current?.focus();
    else document.querySelector<HTMLElement>(`[data-region="${pane.toLowerCase()}"]`)?.focus();
  };
  useLayoutEffect(() => {
    const open = new Set(tabs.map(t => t.id));
    for (const id of states.current.keys()) if (!open.has(id)) states.current.delete(id);
  }, [tabs]);
  useEffect(() => startFrames(config), [config]);
  useEffect(() => {
    const resize = () => setHeight(window.innerHeight);
    window.addEventListener('resize', resize);
    return () => window.removeEventListener('resize', resize);
  }, []);
  useLayoutEffect(() => {
    const key = (e: KeyboardEvent) => {
      if (e.isComposing) return;
      const cmd = e.metaKey || e.ctrlKey;
      if (cmd && e.key === 'Enter') { e.preventDefault(); void runQuery(); }
      else if (e.key === 'Escape') cancel();
      else if (cmd && e.key.toLowerCase() === 't') { e.preventDefault(); openTab(); }
      else if (cmd && e.key.toLowerCase() === 'w') { e.preventDefault(); closeTab(); }
      else if (e.ctrlKey && e.key === 'Tab') { e.preventDefault(); setActive(changeTab(active, tabs.length, e.shiftKey ? -1 : 1)); }
      else if (cmd && e.shiftKey && e.key.toLowerCase() === 'l') { e.preventDefault(); setDark(d => !d); }
      else if (e.key === 'F6') { e.preventDefault(); focusPane(['Explorer', 'Editor', 'Grid'][(Math.max(0, ['Explorer', 'Editor', 'Grid'].indexOf(focus)) + 1) % 3]); }
      else if (e.altKey && ['Digit1', 'Digit2', 'Digit3'].includes(e.code)) { e.preventDefault(); focusPane(['Explorer', 'Editor', 'Grid'][Number(e.code.slice(-1)) - 1]); }
    };
    window.addEventListener('keydown', key, true);
    return () => window.removeEventListener('keydown', key, true);
  });
  useLayoutEffect(() => {
    if (!config.measure) return;
    const rect = (region: string) => {
      const r = document.querySelector(`[data-region="${region}"]`)?.getBoundingClientRect();
      return r ? [r.left, r.top, r.right, r.bottom] : [0, 0, 0, 0];
    };
    const grid = document.querySelector<HTMLElement>('[data-region="grid"]');
    report('snapshot', { ts_ns: wallNs(), focus, sel: selection, tab: active, tabs: tabs.length,
      chars: tab.text.length, tail: tab.text.slice(-120), dark, run, outcome, scroll_row: scrollRow,
      split: editorHeight, sidebar, editor_has_focus: Boolean(view.current?.hasFocus),
      inner: [0, 0, window.innerWidth, window.innerHeight], editor: rect('editor'), grid: rect('grid'),
      splitter: rect('splitter'), sidebar_splitter: rect('sidebar_splitter'), tab0: rect('tab0'), plus: rect('plus'), exp_row1: rect('exp_row1'),
      selected_rect: (() => { const r = document.getElementById(`cell-${selection[0]}-${selection[1]}`)?.getBoundingClientRect(); return r ? [r.left, r.top, r.right, r.bottom] : null; })(),
      cached_tabs: states.current.size,
      cells: document.querySelectorAll('[role="gridcell"]').length,
      dom_elements: document.getElementsByTagName('*').length,
      last_dom_row: [...document.querySelectorAll('[data-row]')].at(-1)?.getAttribute('data-row'),
      scroll_height: grid?.scrollHeight, scroll_top: grid?.scrollTop });
  });
  return <div className={`app ${dark ? 'dark' : 'light'}`} onFocusCapture={e => {
    const target = e.target as HTMLElement;
    if (target.closest('[data-region="editor"]')) setFocus('Editor');
    else if (target.closest('[data-region="grid"]')) setFocus('Grid');
    else if (target.closest('[data-region="explorer"]')) setFocus('Explorer');
    else setFocus('Controls');
  }}>
    <header className="topbar"><div className="brand">d<span>bz</span><small>LAB</small></div><span className="breadcrumb">Workspace <span>/</span> UI stack comparison</span>
      <div className="top-actions"><span className="badge">SYNTHETIC ONLY</span><button title="Toggle theme · ⌘⇧L" onClick={() => setDark(d => !d)}>{dark ? '☀ Light' : '☾ Dark'}</button></div></header>
    <div className="workspace"><div style={{ width: Math.min(sidebar, window.innerWidth * .45) }} className="sidebar"><Explorer /></div>
      <Splitter vertical value={sidebar} min={150} max={450} onChange={setSidebar} />
      <main><div className="tabs" role="tablist" aria-label="SQL tabs">{tabs.map((t, i) => <button role="tab" aria-selected={i === active} key={t.id} data-region={i === 0 ? 'tab0' : undefined} onClick={() => setActive(i)} className={i === active ? 'active' : ''}><span className="sql-icon">SQL</span> Query {t.id}</button>)}<button data-region="plus" onClick={openTab} aria-label="New SQL tab">＋</button><button onClick={closeTab} aria-label="Close active SQL tab" disabled={tabs.length === 1}>×</button></div>
        <div className="query-tools"><button className="primary" onClick={() => void runQuery()} disabled={inFlight.current}>▶ Run <kbd>⌘↵</kbd></button><button onClick={cancel} disabled={!inFlight.current}>■ Cancel</button>
          <span className="read-only">◉ Read-only mock</span><label>Outcome <select aria-label="Mock query outcome" value={outcome} onChange={e => { outcomeRef.current = e.target.value; setOutcome(e.target.value); }}><option value="success">Success</option><option value="empty">Empty</option><option value="error">Error</option></select></label></div>
        <div style={{ height: Math.min(editorHeight, Math.max(80, height - 260)) }} className="editor-pane"><Editor id={tab.id} text={tab.text} dark={dark} states={states.current} onView={v => { view.current = v; }} onChange={text => setTabs(t => t.map(item => item.id === tab.id ? { ...item, text } : item))} /></div>
        <Splitter value={editorHeight} min={80} max={Math.max(80, height - 260)} onChange={setEditorHeight} />
        <div className="result-heading"><strong>Results</strong><span>{visibleRows.toLocaleString()} rows</span><span>20 columns</span><span className="result-meta">On-demand synthetic data · no DB</span></div>
        {run === 'running' || run === 'cancelling' ? <div className="query-message" role="status">{run === 'running' ? 'Running mock query… UI remains interactive' : 'Cancellation requested…'}</div> : null}
        {error && <div className="query-message error" role="alert">{error}</div>}
        <Grid revision={revision} rows={visibleRows} selection={selection} onSelect={setSelection} onScroll={setScrollRow} />
      </main></div>
    <footer role="status"><span><span className="dot" /> Tauri 2 · WKWebView</span><span className="footer-state">{run} {elapsed > 0 && `· ${elapsed.toFixed(0)} ms`}</span><span>{focus} · Row {selection[0] + 1}, Col {selection[1] + 1}</span><span>UTF-8 · SQL / PLSQL</span><span className="muted">T-004a • prototype</span></footer>
  </div>;
}
