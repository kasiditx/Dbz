import { useLayoutEffect, useRef } from 'react';
import { useVirtualizer } from '@tanstack/react-virtual';
import { cell, columnName, columnWidth, contract, navigate, revealRow, type Selection } from './model';

interface Props { revision: number; rows: number; selection: Selection; onSelect: (value: Selection) => void; onScroll: (row: number) => void }
export function Grid({ revision, rows, selection, onSelect, onScroll }: Props) {
  const parent = useRef<HTMLDivElement>(null);
  const vertical = useVirtualizer({ count: rows, getScrollElement: () => parent.current, estimateSize: () => contract.rowHeight, overscan: contract.overscan });
  const horizontal = useVirtualizer({ count: contract.columns, horizontal: true, getScrollElement: () => parent.current, estimateSize: columnWidth, overscan: 1 });
  useLayoutEffect(() => {
    if (parent.current) { parent.current.scrollTop = 0; parent.current.scrollLeft = 0; }
  }, [revision]);
  const vr = vertical.getVirtualItems();
  const vc = horizontal.getVirtualItems();
  const activeId = `cell-${selection[0]}-${selection[1]}`;
  const activeVisible = vr.some(r => r.index === selection[0]) && vc.some(c => c.index === selection[1]);
  return <div ref={parent} className="grid" data-region="grid" role="grid" tabIndex={0}
    aria-label="Synthetic query results" aria-rowcount={rows + 1} aria-colcount={contract.columns}
    aria-activedescendant={activeVisible ? activeId : undefined}
    onScroll={() => onScroll(Math.floor((parent.current?.scrollTop ?? 0) / contract.rowHeight))}
    onCopy={event => { event.preventDefault(); event.clipboardData.setData('text/plain', rows ? cell(...selection) ?? '' : ''); }}
    onKeyDown={event => {
      if (!['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight', 'Home', 'End', 'PageUp', 'PageDown'].includes(event.key)) return;
      event.preventDefault();
      const page = Math.max(1, Math.floor((parent.current?.clientHeight ?? 440) / contract.rowHeight) - 1);
      const next = navigate(selection, event.key, event.metaKey || event.ctrlKey, rows, page);
      onSelect(next);
      vertical.scrollToOffset(revealRow(next[0], parent.current?.scrollTop ?? 0, parent.current?.clientHeight ?? 400));
      horizontal.scrollToIndex(next[1], { align: 'auto' });
    }}>
    <div className="grid-header" role="row" aria-rowindex={1} style={{ width: horizontal.getTotalSize() }}>
      {vc.map(c => <div className="column-header" role="columnheader" aria-colindex={c.index + 1} key={c.index} style={{ left: c.start, width: c.size }}>{columnName(c.index)}<span>↕</span></div>)}
    </div>
    <div className="grid-canvas" style={{ height: vertical.getTotalSize(), width: horizontal.getTotalSize() }}>
      {vr.map(r => <div role="row" aria-rowindex={r.index + 2} className="grid-row" key={r.index} data-row={r.index} style={{ top: r.start, height: r.size, width: horizontal.getTotalSize() }}>
        {vc.map(c => {
          const value = cell(r.index, c.index);
          const selected = r.index === selection[0] && c.index === selection[1];
          return <div role="gridcell" aria-colindex={c.index + 1} aria-selected={selected} id={`cell-${r.index}-${c.index}`} key={c.index}
            className={`grid-cell ${selected ? 'selected' : ''} ${value === null ? 'null' : ''} ${c.index === 0 || c.index === 2 ? 'numeric' : ''}`}
            style={{ left: c.start, width: c.size }}
            onMouseDown={() => { onSelect([r.index, c.index]); parent.current?.focus(); }} title={value ?? 'NULL'}>
            {value === null ? 'NULL' : [...value].slice(0, 90).join('')}
          </div>;
        })}
      </div>)}
    </div>
    {!rows && <div className="empty">No rows returned. Choose Success and run again.</div>}
  </div>;
}
