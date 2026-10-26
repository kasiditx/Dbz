import { describe, expect, it } from 'vitest';
import { cell } from './model';
describe('synthetic workload', () => {
  it('preserves exact decimals, NULL and Thai without materializing a dataset', () => {
    expect(cell(999999, 2)).toBe('7918992081.000999999');
    expect(cell(7, 3)).toBeNull();
    expect(cell(42, 1)).toBe('ลูกค้า 42');
    expect(cell(42, 5)).toBe('2026-01-15T00:00:42.000Z');
    expect(cell(42, 6)!.length).toBeGreaterThan(300);
  });
});

import { navigate, changeTab } from './model';
it('keyboard navigation clamps boundaries and reveals the final row and column', () => {
  expect(navigate([0, 0], 'ArrowUp', false, 1000000, 20)).toEqual([0, 0]);
  expect(navigate([42, 2], 'ArrowDown', true, 1000000, 20)).toEqual([999999, 19]);
  expect(navigate([42, 2], 'Home', false, 1000000, 20)).toEqual([42, 0]);
  expect(navigate([42, 2], 'PageDown', false, 1000000, 20)).toEqual([62, 2]);
  expect(navigate([42, 2], 'ArrowDown', false, 0, 20)).toEqual([0, 0]);
});
it('tab switching wraps forward and backward without changing the count', () => {
  expect(changeTab(0, 3, -1)).toBe(2);
  expect(changeTab(2, 3, 1)).toBe(0);
});
import { revealRow } from './model';
it('reveals intermediate rows below the sticky header, including the last row', () => {
  expect(revealRow(30, 0, 400)).toBe(308);
  expect(revealRow(0, 308, 400)).toBe(0);
  expect(revealRow(999999, 0, 400)).toBe(21999626);
  expect(revealRow(5, 0, 400)).toBe(0);
});
import examples from '../../../benchmarks/t004a/cells.json';
it('matches the language-neutral examples also tested by Rust', () => {
  for (const example of examples) expect(cell(example.row, example.col)).toBe(example.value);
});
