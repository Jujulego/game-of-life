'use client'

import { useWasmMemory, useWasmModule } from '@/hooks/useWasmModule';
import { useEffect, useRef, useState } from 'react';

// Constants
const CELL_SIZE = 5;
const DEAD_COLOR = "#000000";
const ALIVE_COLOR = "#FFFFFF";
const FRAME_RATE = 100;

// Component
export default function Universe() {
  const { Cell, Universe } = useWasmModule();
  const memory = useWasmMemory();

  // State
  const [universe] = useState(() => Universe.random(256, 128));

  // Refs
  const canvas = useRef<HTMLCanvasElement>(null);

  // Effects
  useEffect(() => {
    if (!canvas.current) return;

    // Setup canvas
    const size = universe.size();
    canvas.current.width = size.dx * CELL_SIZE;
    canvas.current.height = size.dy * CELL_SIZE;

    // Animate !
    const ctx = canvas.current.getContext('2d')!;
    let frame: number;
    let last = 0;

    function loop(time: DOMHighResTimeStamp) {
      if (time - last > FRAME_RATE) {
        last = time;

        // Update state
        universe.tick();

        // Draw cells
        const cells = new Uint8Array(memory.buffer, universe.cells(), size.dx * size.dy);

        ctx.beginPath();

        for (let row = 0; row < size.dy; ++row) {
          for (let col = 0; col < size.dx; ++col) {
            const idx = row * size.dx + col;

            ctx.fillStyle = cells[idx] === Cell.Alive ? ALIVE_COLOR : DEAD_COLOR;
            ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
          }
        }

        ctx.stroke();
      }

      frame = requestAnimationFrame(loop);
    }

    frame = requestAnimationFrame(loop);

    return () => cancelAnimationFrame(frame);
  }, [universe]);

  // Render
  return <canvas ref={canvas} />
}
