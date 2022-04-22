/*eslint-disable*/
import dotenv from 'dotenv';
dotenv.config();

import glob from 'glob'
import path from 'path';
import compose from 'docker-compose';
import Context from './utils/context';
import { buildArtifacts, storeCode, instantiateContract, listenForBlockAtHeight } from './utils/utils';
/* eslint-enable */

/* eslint-disable no-console */

async function maybeDeployContracts() {
  const ctx = Context.instance();
  if (process.env.ARTIFACTS_PATH) {
    const artifactsPath = path.resolve(path.normalize(process.env.ARTIFACTS_PATH));
    const wasmPaths = glob.sync(`${artifactsPath}/**/*.wasm`);
    const wallet = ctx.getTestWallet('test1');
    for (let i = 0; i < wasmPaths.length; i++) {
      const fullpath = path.resolve(path.normalize(wasmPaths[i]));
      console.log(`Storing bytecode from ${fullpath}`);
      /* eslint-disable-next-line no-await-in-loop */
      const codeId = await storeCode(ctx.client, wallet, fullpath);
      ctx.addCodeInfo(codeId, fullpath);
    }
  }
}

async function instantiateContracts() {
  const ctx = Context.instance();
  const wallet = ctx.getTestWallet('test1');

  const { codeId } = ctx.getCodeInfo('collateral_manager');
  const contractAddress = await instantiateContract(
    ctx.client,
    wallet,
    codeId,
    {},
  );
  ctx.addContractInfo('collateral_manager', contractAddress, '_main');
}

export default async () => {
  console.log('');

  if (process.env.LOCALTERRA_PATH) {
    console.log('Resetting LocalTerra state...');
    const localterraPath = path.resolve(path.normalize(path.join(process.env.LOCALTERRA_PATH)));
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

  const ctx = Context.instance();

  await listenForBlockAtHeight(1);

  await maybeDeployContracts();
  await instantiateContracts();

  ctx.toFile();
};
/* eslint-enable no-console */
