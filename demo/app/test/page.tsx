import { NoSsr } from '@mui/base';

import GameOfLife from './GameOfLife';

export default function Test() {
  return (
    <NoSsr>
      <GameOfLife />
    </NoSsr>
  );
}
