import { dom, IListenable, multiplexer, once, source } from '@jujulego/event-tree';
import { queryfy } from '@jujulego/utils';

import { measure } from './utils';

// Types
export interface GameOfLifeStateLoading {
  status: 'loading';
  canvas: HTMLCanvasElement;
}

export interface GameOfLifeStateLoaded {
  status: 'loaded';
  canvas: HTMLCanvasElement;
  wasm: typeof import('../wasm/pkg');
}

export interface GameOfLifeStateStarted {
  status: 'started';
  canvas: HTMLCanvasElement;
  context: CanvasRenderingContext2D;
  wasm: typeof import('../wasm/pkg');
  universe: typeof import('../wasm/pkg')['Universe']['prototype'];
}

export interface GameOfLifeStateError {
  status: 'error';
  error: Error;
}

export type GameOfLifeState = GameOfLifeStateLoading
  | GameOfLifeStateLoaded
  | GameOfLifeStateStarted
  | GameOfLifeStateError;

export type GameOfLifeEventMap = {
  loaded: GameOfLifeStateLoaded,
  started: GameOfLifeStateStarted,
  error: GameOfLifeStateError,
}

// Constants
const TICK_RATE = 100;

const wasm$ = queryfy(import('../wasm/pkg'));

// Class
export class GameOfLife implements IListenable<GameOfLifeEventMap> {
  // Attributes
  private _state: GameOfLifeState;

  private readonly _events = multiplexer({
    loaded: source<GameOfLifeStateLoaded>(),
    started: source<GameOfLifeStateStarted>(),
    error: source<GameOfLifeStateError>(),
  });

  // Constructor
  constructor(canvas: HTMLCanvasElement) {
    this._state = { status: 'loading', canvas };

    this._events.on('started', () => this._loop());
    this._events.on('started', (state) => this._followMouse(state));
    this._events.on('error', ({ error }) => console.error(error));

    this._handleWasmLoad(this._state);
  }

  // Methods
  readonly on = this._events.on;
  readonly off = this._events.off;
  readonly clear = this._events.clear;

  start(): void {
    switch (this._state.status) {
      case 'loading':
        once(this._events, 'loaded', (state) => this._handleStart(state));
        break;

      case 'loaded':
        this._handleStart(this._state);
        break;

      default:
        throw new Error(`Cannot start a ${this._state.status} game`);
    }
  }

  stop(): void {
    switch (this._state.status) {
      case 'loading':
      case 'loaded':
        break;

      case 'started':
        this._setState({
          status: 'loaded',
          canvas: this._state.canvas,
          wasm: this._state.wasm,
        });
        break;

      default:
        throw new Error(`Cannot stop a ${this._state.status} game`);
    }
  }

  private _setState(state: Exclude<GameOfLifeState, GameOfLifeStateLoading>) {
    this._state = state;
    this._events.emit(state.status, state);
  }

  private _handleWasmLoad(state: GameOfLifeStateLoading) {
    switch (wasm$.state.status) {
      case 'pending':
        once(wasm$, () => this._handleWasmLoad(state));
        break;

      case 'done':
        this._setState({
          status: 'loaded',
          canvas: state.canvas,
          wasm: wasm$.state.data,
        });
        break;

      case 'failed':
        this._setState({
          status: 'error',
          error: wasm$.state.error,
        });
        break;
    }
  }

  private _handleStart(state: GameOfLifeStateLoaded) {
    // Get canvas context
    const context = state.canvas.getContext('2d');

    if (!context) {
      this._setState({ status: 'error', error: new Error('Unable to get 2d context of canvas') });
      return;
    }

    // Create universe
    const { Universe } = state.wasm;

    this._setState({
      ...state,
      status: 'started',
      context,
      universe: Universe.dead(),
    });
  }

  private _loop() {
    let last = 0;

    const tick = (time: DOMHighResTimeStamp) => {
      if (this._state.status !== 'started') {
        return;
      }

      // Update state
      if (time - last > TICK_RATE) {
        last = time;

        const { context, universe } = this._state;
        measure("game-of-life:tick", () => universe.tick(context));
      }

      requestAnimationFrame(tick);
    };

    requestAnimationFrame(tick);
  }

  private _followMouse({ canvas, context, universe, wasm }: GameOfLifeStateStarted) {
    const { PointInt2D } = wasm;

    // Add cell on mouse move
    let last = 0;

    const off = dom(canvas).on('mousemove', (event) => {
      const now = performance.now();

      if (context && now - last > 10) {
        universe.insert_around(context, new PointInt2D(event.clientX / 5, event.clientY / 5), 3);
        last = now;
      }
    });

    // Unregister on stop
    once(this._events, 'loaded', off);
  }
}
