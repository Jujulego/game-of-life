import { queryfy } from '@jujulego/utils';
import { useQuery } from '@/hooks/useQuery';

const wasm$ = queryfy(import('@/wasm/pkg/wasm'));

export function useWasmModule() {
  return useQuery(wasm$);
}
