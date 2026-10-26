"""Summarize recorded measurements only; missing samples never become zero."""
import csv
import json
import math
from pathlib import Path
from statistics import median

RAW = Path(__file__).resolve().parent / 'raw'

def percentile(values, q):
    ordered = sorted(values)
    return ordered[max(0, math.ceil(len(ordered) * q) - 1)] if ordered else None

def stats(values):
    return {'n': len(values), 'p50': percentile(values, .5), 'p95': percentile(values, .95), 'min': min(values) if values else None, 'max': max(values) if values else None}

def memory(path):
    data = json.loads(path.read_text())
    samples = data['samples']
    valid = [s for s in samples if s['main'] is not None]
    late = [s for s in valid if s['elapsed_s'] >= 60]
    group = lambda s: sum(p['rss_kib'] for p in s.get('attributed', [])) / 1024
    cpu = 0
    for previous, current in zip(valid, valid[1:]):
        old = {p['pid']: p['cpu_s'] for p in previous.get('attributed', [])}
        cpu += sum(max(0, p['cpu_s'] - old.get(p['pid'], p['cpu_s'])) for p in current.get('attributed', []))
    elapsed = valid[-1]['elapsed_s'] - valid[0]['elapsed_s'] if len(valid) > 1 else 0
    footprint_path = path.with_name(path.name.replace('-memory.json', '-footprint.json'))
    footprint = json.loads(footprint_path.read_text()) if footprint_path.exists() else None
    return {'footprint_mib': footprint['total footprint'] / 1048576 if footprint and not footprint.get('errors') else None, 'file': path.name, 'stack': data['metadata']['stack'], 'rows': data['metadata']['rows'],
            'idle_main_mib': median(s['main']['rss_kib'] / 1024 for s in late) if late else None,
            'idle_group_rss_mib': median(group(s) for s in late) if late else None,
            'sampled_peak_group_rss_mib': max(map(group, valid)) if valid else None,
            'sampled_peak_main_mib': max(s['main']['rss_kib'] / 1024 for s in valid) if valid else None,
            'group_cpu_percent': cpu / elapsed * 100 if elapsed else None,
            'seconds': elapsed, 'self_responsible_all': all(s['ownership']['self_responsible'] for s in valid),
            'attributed_pids': sorted({p['pid'] for s in valid for p in s.get('attributed', [])})}

def main():
    output = {'startup': {}, 'idle': [], 'active': []}
    startup = json.loads((RAW / 'startup.json').read_text())
    for stack in ['egui', 'tauri']:
        output['startup'][stack] = stats([r['ready_ms'] for r in startup if r['stack'] == stack])
    output['idle'] = [memory(p) for p in sorted(RAW.glob('idle-*-memory.json'))]
    for path in sorted(RAW.glob('active-*-actions.json')):
        data = json.loads(path.read_text())
        label = data['session']['label']
        item = {'label': label, 'stack': data['session']['stack'], 'rows': data['session']['rows']}
        item['actions'] = {}
        for name in ['grid-key', 'tab-switch', 'large-sql-typing', 'scroll', 'resize']:
            actions = [r for r in data['records'] if r['name'] == name]
            item['actions'][name] = {'attempts': len(actions), 'matched': sum(r['matched'] for r in actions),
                'call_to_state_upper_bound_ms': stats([r['call_to_state_ms'] for r in actions if r['call_to_state_ms'] is not None])}
        frames_path = RAW / Path(data['session']['frames']).name
        if frames_path.exists():
            if frames_path.suffix == '.jsonl':
                frames = [json.loads(l) for l in frames_path.read_text().splitlines() if l.strip()]
            else:
                frames = list(csv.DictReader(frames_path.open()))
            item['frame_interval_ms'] = stats([float(f['interval_ms']) for f in frames if f.get('phase', 'external') == 'external'])
            item['window_widths'] = sorted({float(f['win_w']) for f in frames})
            item['resize_width_changes'] = sum(float(a['win_w']) != float(b['win_w']) for a, b in zip(frames, frames[1:]))
            item['max_cells'] = max((int(f['cells']) for f in frames), default=None)
            item['max_rows'] = max((int(f['vis_rows']) for f in frames), default=None)
            if frames_path.suffix == '.jsonl': item['max_dom_elements'] = max((f['dom_elements'] for f in frames), default=None)
        mem = RAW / f'{label}-memory.json'
        if mem.exists():
            item['memory'] = memory(mem)
            scroll = [r for r in data['records'] if r['name'] == 'scroll']
            samples = json.loads(mem.read_text())['samples']
            during_scroll = [s for s in samples if scroll and min(r['start_ns'] for r in scroll) <= s.get('wall_ns', 0) <= max(r['end_ns'] for r in scroll)]
            item['scroll_memory_samples'] = len(during_scroll)
            item['scroll_sampled_peak_group_rss_mib'] = max((sum(p['rss_kib'] for p in s['attributed']) / 1024 for s in during_scroll), default=None)
            item['scroll_sampled_peak_main_mib'] = max((s['main']['rss_kib'] / 1024 for s in during_scroll if s['main']), default=None)
        output['active'].append(item)
    (RAW / 'summary.json').write_text(json.dumps(output, indent=2))
    print(json.dumps(output, indent=2))

if __name__ == '__main__': main()
