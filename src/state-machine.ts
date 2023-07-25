import { EventData, EventKey, groupMap, IListenable, IObservable, source } from '@jujulego/event-tree';

export type StateMap = Record<string, unknown>;

export type StateMachineState<M extends StateMap> = { [K in EventKey<M>]: { key: K, data: EventData<M, K> } }[EventKey<M>];

export type StateListenerMap<M extends StateMap> = {
  [K in keyof M]?: (state: M[K]) => StateMachineState<M>;
};

export interface StateMachine<M extends StateMap> extends IObservable<EventData<M>>, IListenable<M> {
  state: StateMachineState<M>;
  clear(key?: EventKey<M>): void;
}

export function stateMachine<M extends StateMap>(listeners: StateListenerMap<M>, initial: StateMachineState<M>): StateMachine<M> {
  const events = groupMap(() => source<EventData<M>>());

  const machine = {
    state: initial,

    on: events.on,
    off: events.off,
    clear: events.clear,

    subscribe: events.subscribe,
    unsubscribe: events.unsubscribe,
  };

  // Initiate machine
  for (const key of Object.keys(listeners) as (EventKey<M>)[]) {
    const listener = listeners[key];

    if (listener) {
      events.on(key, (state) => {
        const result = listener(state);

        if (result) {
          machine.state = result;
          events.emit(result.key, result.data);
        }
      });
    }
  }

  events.emit(initial.key, initial.data);

  return machine;
}
