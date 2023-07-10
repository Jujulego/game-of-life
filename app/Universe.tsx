'use client'

import { MouseEvent, useCallback, useEffect, useRef, useState } from 'react';
import { useWasmModule } from '@/hooks/useWasmModule';

// Constants
const CELL_SIZE = 5;
const TICK_RATE = 100;
const FRAME_RATE = 25;

// Component
export default function Universe() {
  const { Universe, UniverseStyle } = useWasmModule();

  // State
  const [universe] = useState(() => Universe.dead(256, 128));

  // Refs
  const canvas = useRef<HTMLCanvasElement>(null);

  // Effects
  useEffect(() => {
    if (!canvas.current) return;

    // Setup canvas
    const size = universe.size;

    canvas.current.width = size.dx * CELL_SIZE;
    canvas.current.height = size.dy * CELL_SIZE;

    // Setup universe
    universe.style = UniverseStyle.dark();

    // Animate !
    const ctx = canvas.current.getContext('2d')!;

    let frame: number;
    let lastTick = 0;
    let lastFrame = 0;

    function tick(time: DOMHighResTimeStamp) {
      // Update state
      if (time - lastTick > TICK_RATE) {
        lastTick = time;
        universe.tick();
      }

      // Render state
      if (time - lastFrame > FRAME_RATE) {
        lastFrame = time;
        universe.render(ctx);
      }

      frame = requestAnimationFrame(tick);
    }

    frame = requestAnimationFrame(tick);

    return () => cancelAnimationFrame(frame);
  }, [universe]);

  // Callbacks
  const last = useRef(0);

  const handleMove = useCallback((event: MouseEvent<HTMLCanvasElement>) => {
    const now = performance.now();

    if (now - last.current > 10) {
      universe.insert_around(event.clientX / 5, event.clientY / 5, 3);
      last.current = now;
    }
  }, [universe]);

  // Render
  return <canvas ref={canvas} onMouseMove={handleMove} />
}
