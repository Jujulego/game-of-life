'use client'

import { MouseEvent, TouchEvent, useCallback, useEffect, useRef, useState } from 'react';
import { useWasmModule } from '@/hooks/useWasmModule';

// Constants
const TICK_RATE = 100;

// Utils
function measure(name: string, fn: () => void) {
  performance.mark(`${name}-start`);

  fn();

  performance.mark(`${name}-end`);
  performance.measure(name, `${name}-start`, `${name}-end`);
}

// Component
export default function Universe() {
  const { PointInt2D, Universe, UniverseStyle, VectorInt2D } = useWasmModule();

  // State
  const [universe] = useState(() => Universe.dead());
  const [context, setContext] = useState<CanvasRenderingContext2D | null>(null);

  // Refs
  const canvas = useRef<HTMLCanvasElement>(null);

  // Effects
  useEffect(() => {
    if (!canvas.current) return;

    // Initiate context
    let ctx = canvas.current.getContext('2d')!;
    setContext(ctx);

    const height = canvas.current.height = canvas.current.parentElement!.clientHeight;
    const width = canvas.current.width = canvas.current.parentElement!.clientWidth;

    universe.style = UniverseStyle.dark();
    measure("set_update_area", () => universe.set_update_area(new PointInt2D(-5, -5), new PointInt2D(width / 5 + 5, height / 5 + 5)));
    universe.redraw(ctx, new VectorInt2D(width, height));

    // Follow container size
    const observer = new ResizeObserver((entries) => {
      if (entries.length === 0) return;
      if (!canvas.current) return;

      const { height, width } = entries[0].contentRect;

      canvas.current.height = height;
      canvas.current.width = width;
      measure("set_update_area", () => universe.set_update_area(new PointInt2D(-5, -5), new PointInt2D(width / 5 + 5, height / 5 + 5)));
      universe.redraw(ctx!, new VectorInt2D(width, height));
    });

    observer.observe(canvas.current.parentElement!);

    return () => observer.disconnect();
  }, [universe]);

  useEffect(() => {
    if (!context || !canvas.current) return;

    // Animate !
    let frame: number;
    let lastTick = 0;

    function tick(time: DOMHighResTimeStamp) {
      // Update state
      if (context && time - lastTick > TICK_RATE) {
        lastTick = time;

        measure("tick", () => universe.tick(context));
      }

      frame = requestAnimationFrame(tick);
    }

    frame = requestAnimationFrame(tick);

    return () => cancelAnimationFrame(frame);
  }, [context, universe]);

  // Callbacks
  const last = useRef(0);

  const handleMove = useCallback((event: MouseEvent<HTMLCanvasElement>) => {
    const now = performance.now();

    if (context && now - last.current > 10) {
      universe.insert_around(context, new PointInt2D(event.clientX / 5, event.clientY / 5), 3);
      last.current = now;
    }
  }, [context, universe]);

  const handleTouch = useCallback((event: TouchEvent<HTMLCanvasElement>) => {
    const now = performance.now();

    if (context && now - last.current > 10) {
      for (let idx = 0; idx < event.changedTouches.length; idx++) {
        const touch = event.changedTouches[idx];
        universe.insert_around(context, new PointInt2D(touch.clientX / 5, touch.clientY / 5), 3);
      }
      last.current = now;
    }
  }, [context, universe]);

  // Render
  return <canvas
    ref={canvas}
    onMouseMove={handleMove}
    onTouchMove={handleTouch}
    style={{ display: 'block' }}
  />;
}
