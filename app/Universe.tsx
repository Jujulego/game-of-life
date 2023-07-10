'use client'

import { MouseEvent, useCallback, useEffect, useRef, useState } from 'react';
import { useWasmModule } from '@/hooks/useWasmModule';

// Constants
const TICK_RATE = 100;

// Component
export default function Universe() {
  const { Universe, UniverseStyle } = useWasmModule();

  // State
  const [universe] = useState(() => Universe.dead(256, 128));
  const [context, setContext] = useState<CanvasRenderingContext2D | null>(null);

  // Refs
  const canvas = useRef<HTMLCanvasElement>(null);

  // Effects
  useEffect(() => {
    if (!canvas.current) return;

    // Initiate context
    let ctx = canvas.current.getContext('2d')!;
    setContext(ctx);

    canvas.current.height = canvas.current.parentElement!.clientHeight;
    canvas.current.width = canvas.current.parentElement!.clientWidth;

    universe.style = UniverseStyle.dark();
    universe.redraw(ctx, canvas.current.width, canvas.current.height);

    // Follow container size
    const observer = new ResizeObserver((entries) => {
      if (entries.length === 0) return;
      if (!canvas.current) return;

      const { height, width } = entries[0].contentRect;

      canvas.current.height = height;
      canvas.current.width = width;
      universe.redraw(ctx!, width, height);
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

        performance.mark("tick-start");

        universe.tick(context);

        performance.mark("tick-end");
        performance.measure("tick", "tick-start", "tick-end");
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
      universe.insert_around(context, event.clientX / 5, event.clientY / 5, 3);
      last.current = now;
    }
  }, [context, universe]);

  // Render
  return <canvas ref={canvas} onMouseMove={handleMove} />;
}
