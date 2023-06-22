import { queryfy } from '@jujulego/utils';
import { useQuery } from '@/hooks/useQuery';

const wasm$ = queryfy(import('@/wasm/pkg/wasm'));
const wasmBg$ = queryfy(import('@/wasm/pkg/wasm_bg.wasm'));

export function useWasmModule() {
  return useQuery(wasm$);
}

export function useWasmMemory() {
  return useQuery(wasmBg$).memory;
}
