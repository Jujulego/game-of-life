/**
 * Measures duration of "fn" using the performance api
 * @param name
 * @param fn
 */
export function measure(name: string, fn: () => void) {
  performance.mark(`${name}-start`);

  fn();

  performance.mark(`${name}-end`);
  performance.measure(name, `${name}-start`, `${name}-end`);
}
