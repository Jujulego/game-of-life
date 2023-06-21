'use client'

import { Query, queryfy } from '@jujulego/utils';

const wasmQuery = queryfy(import('@/wasm'))

function useQuery<T>(query: Query<T>): T {
  const state = query.state;

  switch (state.status) {
    case 'pending':
      throw query;

    case 'failed':
      throw state.error;

    case 'done':
      return state.data;
  }
}

export default function GreetButton() {
  const { greet } = useQuery(wasmQuery);

  return <button onClick={greet}>Hello !</button>
}
