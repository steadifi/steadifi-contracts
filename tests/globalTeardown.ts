/*eslint-disable*/
import dotenv from 'dotenv';
dotenv.config();

import path from 'path';
import compose from 'docker-compose';
/* eslint-enable */

/* eslint-disable no-console */
export default async () => {
  console.log('');

  if (process.env.LOCALTERRA_PATH) {
    const localterraPath = path.resolve(path.normalize(path.join(process.env.LOCALTERRA_PATH)));

    console.log('Stopping LocalTerra...');
    await compose.stop({ cwd: localterraPath });

    console.log('Resetting LocalTerra state...');
    await compose.rm({ cwd: localterraPath });
  }
};
/* eslint-enable no-console */
