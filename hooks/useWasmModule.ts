import { queryfy } from '@jujulego/utils';

const wasm$ = queryfy(import('@/wasm/pkg/wasm'));
const wasmBg$ = queryfy(import('@/wasm/pkg/wasm_bg.wasm'));

export function useWasmModule() {
  const state = wasm$.state;

  switch (state.status) {
    case 'pending':
      throw wasm$;

    case 'failed':
      throw state.error;

    case 'done':
      return state.data;
  }
}

export function useWasmMemory() {
  const state = wasmBg$.state;

  switch (state.status) {
    case 'pending':
      throw wasm$;

    case 'failed':
      throw state.error;

    case 'done':
      return state.data.memory;
  }
}
