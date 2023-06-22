'use client'

import { useWasmModule } from '@/hooks/useWasmModule';

export default function GreetButton() {
  const { greet } = useWasmModule();

  return <button onClick={greet}>Hello !</button>
}
