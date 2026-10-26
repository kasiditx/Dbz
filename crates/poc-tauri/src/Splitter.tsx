import { useRef } from 'react';
interface Props { vertical?: boolean; value: number; min: number; max: number; onChange: (value: number) => void }
export function Splitter({ vertical = false, value, min, max, onChange }: Props) {
  const drag = useRef<{ start: number; value: number } | null>(null);
  const clamp = (v: number) => Math.min(max, Math.max(min, v));
  return <div role="separator" aria-label={vertical ? 'Explorer width' : 'Editor height'}
    aria-orientation={vertical ? 'vertical' : 'horizontal'} aria-valuenow={Math.round(value)} aria-valuemin={min} aria-valuemax={max}
    tabIndex={0} className={`splitter ${vertical ? 'vertical' : 'horizontal'}`} data-region={vertical ? 'sidebar_splitter' : 'splitter'}
    onPointerDown={e => { e.currentTarget.setPointerCapture(e.pointerId); drag.current = { start: vertical ? e.clientX : e.clientY, value }; }}
    onPointerMove={e => { if (drag.current) onChange(clamp(drag.current.value + (vertical ? e.clientX : e.clientY) - drag.current.start)); }}
    onPointerUp={() => { drag.current = null; }} onPointerCancel={() => { drag.current = null; }}
    onKeyDown={e => {
      if (['ArrowLeft', 'ArrowUp', 'ArrowRight', 'ArrowDown'].includes(e.key)) {
        e.preventDefault(); onChange(clamp(value + (['ArrowLeft', 'ArrowUp'].includes(e.key) ? -10 : 10)));
      }
    }} />;
}
