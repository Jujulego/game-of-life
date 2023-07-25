import { stateMachine } from '../src/state-machine';

// Type
type TestMachine = {
  str: string,
  num: number,
  bool: boolean,
}

// Tests
describe('stateMachine', () => {
  it('should call str listener', () => {
    const spy = jest.fn();
    const machine = stateMachine<TestMachine>(
      { str: spy },
      { key: 'str', data: 'life' }
    );

    expect(machine.state).toEqual({ key: 'str', data: 'life' });
    expect(spy).toHaveBeenCalledWith('life');
  });

  it('should call num listener', () => {
    const spy = jest.fn();
    const machine = stateMachine<TestMachine>({
      str: () => ({ key: 'num', data: 42 }),
      num: spy,
    }, { key: 'str', data: 'life' });

    expect(machine.state).toEqual({ key: 'num', data: 42 });
    expect(spy).toHaveBeenCalledWith(42);
  });
});
