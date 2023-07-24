import { queryfy } from '@jujulego/utils';
import { useQuery } from '@/hooks/useQuery';

const wasm$ = queryfy(import('../../wasm/pkg'));

wasm$.on('done', ({ data: mod  }) => mod.set_panic_hook());

export function useWasmModule() {
  return useQuery(wasm$);
}
