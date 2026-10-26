import { invoke } from '@tauri-apps/api/core';
export interface Config { rows: number; largeSql: boolean; measure: boolean; seconds: number }
export const wallNs = () => Math.round((performance.timeOrigin + performance.now()) * 1e6);
export function report(command: string, value: unknown) {
  void invoke(command, { value }).catch(error => console.error('Prototype diagnostics failed', String(error)));
}
/** rAF is browser scheduling, not presentation. No animation loop in unmeasured/idle mode. */
export function startFrames(config: Config) {
  let frame = 0;
  let previous = 0;
  let handle = 0;
  const start = performance.now();
  const samples: object[] = [];
  function tick(now: number) {
    frame++;
    if (frame === 6) void invoke('ready');
    if (config.measure && config.seconds > 0 && frame > 30) {
      samples.push({ ts_ns: wallNs(), frame, interval_ms: now - previous,
        cells: document.querySelectorAll('[role="gridcell"]').length,
        vis_rows: document.querySelectorAll('[data-row]').length,
        dom_elements: document.getElementsByTagName('*').length,
        win_w: window.innerWidth, win_h: window.innerHeight });
    }
    previous = now;
    if (samples.length >= 20000 || (config.seconds > 0 && now - start > config.seconds * 1000)) {
      report('frames', samples);
      return;
    }
    if (frame < 6 || (config.measure && config.seconds > 0)) handle = requestAnimationFrame(tick);
  }
  handle = requestAnimationFrame(tick);
  return () => cancelAnimationFrame(handle);
}
