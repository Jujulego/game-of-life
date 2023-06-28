'use client'

import { useWasmModule } from '@/hooks/useWasmModule';
import { useEffect, useRef, useState } from 'react';

// Constants
const CELL_SIZE = 5;
const FRAME_RATE = 100;

// Component
export default function Universe() {
  const { Universe, UniverseStyle } = useWasmModule();

  // State
  const [universe] = useState(() => Universe.fixed(256, 128));

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
    let last = 0;

    function loop(time: DOMHighResTimeStamp) {
      if (time - last > FRAME_RATE) {
        last = time;

        performance.mark('loop-start');

        // Update state
        universe.tick();
        universe.render(ctx);

        performance.mark('loop-end');
        performance.measure('loop', 'loop-start', 'loop-end');
      }

      frame = requestAnimationFrame(loop);
    }

    frame = requestAnimationFrame(loop);

    return () => cancelAnimationFrame(frame);
  }, [universe]);

  // Render
  return <canvas ref={canvas} />
}
