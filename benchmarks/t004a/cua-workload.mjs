/** Execute inside cua_repl with its documented native App API. No alternate input driver.
 * UI geometry comes from actual app snapshots and screenshots. Actions are identical per stack.
 * Transport-inclusive timing is an upper bound, NOT exact CGEvent dispatch or display latency.
 */
function dimensions(bytes) {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  if (view.getUint32(0) === 0x89504e47) return [view.getUint32(16), view.getUint32(20)];
  if (view.getUint16(0) !== 0xffd8) throw new Error('Unsupported screenshot encoding');
  let offset = 2;
  while (offset + 9 < bytes.length) {
    if (bytes[offset] !== 0xff) throw new Error('Invalid JPEG marker');
    const marker = bytes[offset + 1];
    const length = view.getUint16(offset + 2);
    if ([0xc0, 0xc1, 0xc2].includes(marker)) return [view.getUint16(offset + 7), view.getUint16(offset + 5)];
    offset += 2 + length;
  }
  throw new Error('Screenshot dimensions missing');
}

export async function runPhase(app, fs, directory, phase) {
  const session = JSON.parse(await fs.readFile(`${directory}/session.json`, 'utf8'));
  const output = `${directory}/${session.label}-actions.json`;
  const records = phase === 'grid-key' ? [] : JSON.parse(await fs.readFile(output, 'utf8')).records;
  const states = async () => {
    const text = await fs.readFile(session.events, 'utf8');
    // The writer may be midway through the last JSONL record. Only newline-terminated records are complete.
    return text.slice(0, text.lastIndexOf('\n') + 1).split('\n').filter(Boolean).map(line => JSON.parse(line));
  };
  const latest = async () => (await states()).at(-1);
  const save = () => fs.writeFile(`${directory}/${session.label}-actions.json`, JSON.stringify({ session, records }, null, 2));
  const snapshot = () => app.getAXState({ emit: false });
  const now = () => Date.now() * 1e6;
  const pending = [];
  const mark = async (name, action, matches) => {
    const before = await latest();
    const start_ns = now();
    await action();
    const end_ns = now();
    const evidence = (await states()).find(s => s.ts_ns >= start_ns && matches(s, before));
    pending.push({ index: records.length, matches });
    records.push({ name, start_ns, end_ns, matched: Boolean(evidence),
      call_to_state_ms: evidence ? (evidence.ts_ns - start_ns) / 1e6 : null,
      before, after: await latest() });
    await save();
  };
  await snapshot();
  if (phase === 'grid-key') {
  await app.pressKey('alt+3'); await app.pressKey('super+Up'); await snapshot();
  for (let i = 0; i < 20; i++) await mark('grid-key', () => app.pressKey('Down'), (s, p) => s.sel[0] === p.sel[0] + 1);
  }
  if (phase === 'tab-switch') for (let i = 0; i < 20; i++) await mark('tab-switch', () => app.pressKey('ctrl+Tab'), (s, p) => s.tab === (p.tab + 1) % p.tabs);
  if (phase === 'large-sql-typing') {
  await app.pressKey('alt+2'); await snapshot();
  for (let i = 0; i < 20; i++) await mark('large-sql-typing', () => app.typeText('a'), (s, p) => s.chars === p.chars + 1);
  }
  if (phase === 'scroll' && !records.some(r => r.name === 'scroll')) {
    await app.pressKey('alt+3'); await app.pressKey('super+Up'); await snapshot();
  }
  let s = await latest();
  // Both captures are Retina 2x, and native title bar is 32 logical points; these
  // are verified from the initial actual screenshot, not inferred from CSS alone.
  const initial = await app.getScreenshot({ emit: false });
  const [width, height] = dimensions(initial);
  const contentWidth = s.inner[2] - s.inner[0];
  const contentHeight = s.inner[3] - s.inner[1];
  const scale = width / contentWidth;
  const chrome = height - contentHeight * scale;
  const point = [((s.grid[0] + s.grid[2]) / 2) * scale, ((s.grid[1] + s.grid[3]) / 2) * scale + chrome];
  if (phase === 'scroll') for (let i = records.filter(r => r.name === 'scroll').length; i < 20; i++) await mark('scroll', () => app.scroll(point, 'down', 1), (next, previous) => next.scroll_row > previous.scroll_row);
  if (phase === 'resize') for (let i = 0; i < 6; i++) {
    const image = await app.getScreenshot({ emit: false });
    const [w, h] = dimensions(image);
    const direction = i % 2 === 0 ? -1 : 1;
    await mark('resize', () => app.drag([w - 2, h / 2], [w - 2 + direction * 160 * scale, h / 2]),
      (next, previous) => next.inner[2] - next.inner[0] !== previous.inner[2] - previous.inner[0]);
  }
  await snapshot();
  const completeStates = await states();
  for (const { index, matches } of pending) {
    const record = records[index];
    const nextStart = records[index + 1]?.start_ns ?? now();
    const evidence = completeStates.find(state => state.ts_ns >= record.start_ns && state.ts_ns < nextStart && matches(state, record.before));
    record.matched = Boolean(evidence);
    record.call_to_state_ms = evidence ? (evidence.ts_ns - record.start_ns) / 1e6 : null;
    if (evidence) record.after = evidence;
  }
  s = await latest();
  await fs.writeFile(`${directory}/${session.label}.png`, await app.getScreenshot({ emit: false }));
  await fs.writeFile(`${directory}/${session.label}-ax.txt`, await app.getAXState({ emit: false, disableDiffing: true }));
  await save();
  return { label: session.label, actions: records.length, matched: records.filter(r => r.matched).length,
    geometry: { width, height, scale, chrome }, last: s };
}
