import { queryfy } from '@jujulego/utils';

const wasm$ = queryfy(import('@/wasm'));

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
