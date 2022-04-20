/*eslint-disable*/
import dotenv from 'dotenv';
dotenv.config();

import glob from 'glob'
import path from 'path';
import compose from 'docker-compose';
import Context from './utils/context';
import { buildArtifacts, storeCode } from './utils/utils';
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

  if (!process.env.DISABLE_REBUILD || process.env.DISABLE_REBUILD === 'FALSE') {
    console.log('Building artifacts...');
    buildArtifacts();
  } else if (process.env.DISABLE_REBUILD !== 'TRUE') {
    throw new TypeError(`Cannot recognize environment variable DISABLE_REBUILD=${process.env.DISABLE_REBUILD}.`
        + " Only acceptable values are 'TRUE' and 'FALSE'.");
  }

  // TODO: Wait till block 1 has been produced

  const ctx = Context.instance();

  if (process.env.ARTIFACTS_PATH) {
    const artifactsPath = path.normalize(process.env.ARTIFACTS_PATH);
    const wasmPaths = glob.sync(`${artifactsPath}/**/*.wasm`);
    const wallet = ctx.getTestWallet('test1');
    for (let i = 0; i < wasmPaths.length; i++) {
      const fullpath = path.normalize(wasmPaths[i]);
      /* eslint-disable-next-line no-await-in-loop */
      const codeId = await storeCode(ctx.client, wallet, fullpath);
      ctx.addCodeInfo(codeId, fullpath);
    }
  }

  ctx.toFile();
};
/* eslint-enable no-console */
