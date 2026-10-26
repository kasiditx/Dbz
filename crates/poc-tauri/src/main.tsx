import { createRoot } from 'react-dom/client';
import { invoke } from '@tauri-apps/api/core';
import { App } from './App';
import type { Config } from './diagnostics';
import './style.css';
const root = createRoot(document.getElementById('root')!);
root.render(<div className="boot">Loading local prototype…</div>);
invoke<Config>('configuration').then(config => root.render(<App config={config} />)).catch(() => {
  root.render(<div role="alert">Cannot load native prototype configuration. Launch the Tauri application.</div>);
});
