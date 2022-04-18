// Helper functions to deploy contract
// Helper functions to instantiate contract
// Get owner of contract
// query contract
// execute functions of contract
// Transfer ownership of contract
// Make contract use different code by changing the code ID (aka migrating)

import path from 'path';
import { execSync } from 'child_process';
import { LocalTerra, LCDClient } from '@terra-money/terra.js';

export function createLCDClient():LCDClient {
  if (process.env.USE_LOCAL_DEFAULT === 'TRUE') {
    return new LocalTerra();
  } if (process.env.USE_LOCAL_DEFAULT === 'FALSE') {
    return new LCDClient({
      URL: process.env.CLIENT_URL as string,
      chainID: process.env.CHAIN_ID as string,
    });
  }
  throw new TypeError(`Cannot recognize environment variable USE_LOCAL_DEFAULT=${process.env.USE_LOCAL_DEFAULT}.`
      + " Only acceptable values are 'TRUE' and 'FALSE'.");
}

export function buildArtifacts() {
  const buildScriptPath = path.normalize(path.join(process.env.SCRIPTS_PATH as string, 'build_release.sh'));
  execSync(buildScriptPath);
}
