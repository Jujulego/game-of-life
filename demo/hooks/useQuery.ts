import { Query } from '@jujulego/utils';

export function useQuery<T>(query: Query<T>): T {
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
