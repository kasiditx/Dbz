import { useState } from 'react';
export function Explorer() {
  const [expanded, setExpanded] = useState(true);
  const [selected, setSelected] = useState(0);
  const nodes = expanded ? ['APP', 'Tables', 'ORDERS', 'CUSTOMERS', 'Views', 'Packages'] : ['APP'];
  const toggle = () => { setExpanded(!expanded); setSelected(0); };
  return <aside className="explorer"><div className="section-label">EXPLORER <span>MOCK</span></div>
    <div className="connection"><span className="dot" /> Local sandbox <small>Synthetic · no connection</small></div>
    <div role="tree" aria-label="Mock database explorer" tabIndex={0} data-region="explorer" aria-activedescendant={`node-${selected}`}
      onKeyDown={e => {
        if (e.key.startsWith('Arrow') || e.key === 'Enter') e.preventDefault();
        if (e.key === 'ArrowDown') setSelected(s => Math.min(nodes.length - 1, s + 1));
        if (e.key === 'ArrowUp') setSelected(s => Math.max(0, s - 1));
        if (e.key === 'ArrowLeft') { setExpanded(false); setSelected(0); }
        if (e.key === 'ArrowRight') setExpanded(true);
        if (e.key === 'Enter' && selected === 0) toggle();
      }}>
      {nodes.map((name, i) => <div key={name} role="treeitem" id={`node-${i}`} aria-selected={selected === i}
        aria-level={i === 0 ? 1 : i === 2 || i === 3 ? 3 : 2} aria-expanded={i === 0 ? expanded : undefined}
        data-region={i === 1 ? 'exp_row1' : undefined} className={`tree-node ${selected === i ? 'active' : ''}`}
        style={{ paddingLeft: i === 0 ? 14 : i === 2 || i === 3 ? 46 : 30 }}
        onClick={e => { setSelected(i); e.currentTarget.parentElement?.focus(); if (i === 0) toggle(); }}>
        <span className="tree-icon">{i === 0 ? expanded ? '⌄' : '›' : i === 2 || i === 3 ? '▦' : '◇'}</span>{name}
      </div>)}
    </div><div className="sidebar-note">T-004a experiment<br />20 columns · generated on demand</div>
  </aside>;
}
