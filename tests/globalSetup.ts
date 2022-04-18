/*eslint-disable*/
import dotenv from 'dotenv';
dotenv.config();

import path from 'path';
import compose from 'docker-compose';
import Context from './utils/context';
import { buildArtifacts } from './utils/utils';
/* eslint-enable */

/* eslint-disable no-console */
export default async () => {
  console.log('');

  if (process.env.LOCALTERRA_PATH) {
    console.log('Resetting LocalTerra state...');
    const localterraPath = path.normalize(path.join(process.env.LOCALTERRA_PATH));
    await compose.rm({ cwd: localterraPath });

    console.log('Starting LocalTerra...');
    await compose.upAll({ cwd: localterraPath });
  }

  console.log('Building artifacts...');
  buildArtifacts();

  const ctx = Context.instance();
  // Deploy contracts

  ctx.toFile();
};
/* eslint-enable no-console */
