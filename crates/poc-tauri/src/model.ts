import contract from '../../../benchmarks/t004a/workload.json';
import query from '../../../benchmarks/t004a/query.sql?raw';
export { contract };
export const sqlText = (large: boolean) => query.repeat(large ? contract.largeSqlRepeats : 1);
export const columnWidth = (col: number) => col === contract.wideColumn ? contract.wideColumnWidth : contract.columnWidth;
export const columnName = (col: number) => ['ID', 'CUSTOMER_NAME', 'AMOUNT', 'STATUS', 'NOTES', 'TIMESTAMP', 'LONG_TEXT'][col] ?? `COL_${col}`;

/** Exact decimals and synthetic strings generated only for requested cells. */
export function cell(row: number, col: number): string | null {
  switch (col) {
    case 0: return String(row);
    case 1: return `ลูกค้า ${row}`;
    case 2: return `${row * 7919}.${String(row % 1_000_000_000).padStart(9, '0')}`;
    case 3: return row % 7 === 0 ? null : ['OPEN', 'PAID', 'VOID'][row % 3];
    case 4: return `หมายเหตุยาว ${row} `.repeat(30);
    case 5: return `2026-01-15T${[Math.floor(row / 3600) % 24, Math.floor(row / 60) % 60, row % 60].map(n => String(n).padStart(2, '0')).join(':')}.000Z`;
    case 6: return `Long Unicode Ω 日本語 ${row} `.repeat(30);
    default: return `r${row}c${col}`;
  }
}

export type Selection = readonly [number, number];
export function navigate([row, col]: Selection, key: string, command: boolean, rows: number, page: number): Selection {
  if (!rows) return [0, 0];
  const last = rows - 1;
  if (command && (key === 'ArrowUp' || key === 'Home')) return [0, 0];
  if (command && (key === 'ArrowDown' || key === 'End')) return [last, contract.columns - 1];
  switch (key) {
    case 'ArrowUp': return [Math.max(0, row - 1), col];
    case 'ArrowDown': return [Math.min(last, row + 1), col];
    case 'ArrowLeft': return [row, Math.max(0, col - 1)];
    case 'ArrowRight': return [row, Math.min(contract.columns - 1, col + 1)];
    case 'PageUp': return [Math.max(0, row - page), col];
    case 'PageDown': return [Math.min(last, row + page), col];
    case 'Home': return [row, 0];
    case 'End': return [row, contract.columns - 1];
    default: return [row, col];
  }
}
export const changeTab = (active: number, count: number, delta: number) => (active + count + delta) % count;

export const headerHeight = 26;
/** The sticky header occupies viewport space but also precedes the row canvas. */
export function revealRow(row: number, scrollTop: number, viewportHeight: number): number {
  const top = row * contract.rowHeight;
  const bottom = top + contract.rowHeight + headerHeight;
  if (top < scrollTop) return top;
  return bottom > scrollTop + viewportHeight ? bottom - viewportHeight : scrollTop;
}
