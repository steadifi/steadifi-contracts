// query contract
// execute functions of contract
// Transfer ownership of contract
// Make contract use different code by changing the code ID (aka migrating)
// Query non-native token balance
import fs from 'fs';
import path from 'path';
import { execSync } from 'child_process';
import {
  LocalTerra, LCDClient, Wallet, Msg, MsgStoreCode, MsgInstantiateContract,
  Fee, Int, Dec, Numeric, Coin, Coins, isTxError, WebSocketClient,
} from '@terra-money/terra.js';
import { getMnemonicKey, TestAccountName } from './testAccounts';
import TxResult from './txResult';

function isWallet(obj: Wallet | string): obj is Wallet {
  return (obj as Wallet).key !== undefined;
}

/**
 * Creates an LCDClient based on environment variables defined. If USE_LOCAL_DEFAULT
 * is set to TRUE, a LocalTerra instance is returned (which is also an LCDClient).
 * Otherwise, CLIENT_URL and CHAIN_ID are used to create an LCDClient
 * @return an LCDClient
 */
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

/**
 * Runs the build script `build_release.sh` located in the path defined by
 * the environment variable SCRIPTS_PATH
 * @return void
 */
export function buildArtifacts() {
  const buildScriptPath = path.resolve(path.normalize(path.join(process.env.SCRIPTS_PATH as string, 'build_release.sh')));
  execSync(buildScriptPath);
}

/**
 * Uses websockets to listen for new blocks. The promise resolves when a block height
 * greater or equal than blockHeight is reached
 *
 * example usage:
 * await listenForBlockAtHeight(10);
 *
 * The above example will wait until block 10 has been reached
 * @param  blockHeight                Block height to look for
 * @param  websocketUrl               websocket URL to connect. Default is for LocalTerra
 * @return The block height that resolved the promise.
 */
export async function listenForBlockAtHeight(blockHeight: number, websocketUrl:string = 'ws://localhost:26657/websocket') {
  return new Promise((resolve) => {
    const wsclient = new WebSocketClient(websocketUrl, 3);
    wsclient.subscribe('NewBlockHeader', {}, (data) => {
      if (parseInt(data.value.header.height, 10) >= blockHeight) {
        wsclient.destroy();
        resolve(parseInt(data.value.header.height, 10));
      }
    });
    wsclient.start();
  });
}

/**
 * Gives instances to wallets predefined in 'testAccounts.ts'. On LocalTerra, these
 * wallets already contain a small amount of LUNA, useful for testing
 * @param  client               the active LCDClient
 * @param  name                 name of the test account. Consult 'testAccounts.ts' for
 * for all allowed names
 * @return        a Wallet instance
 */
export function getTestWallet(client: LCDClient, name: TestAccountName) {
  return client.wallet(getMnemonicKey(name));
}

/**
 * Convert an amount of coins to microUnits. This essentially multiplies the number
 * given (in string format) by 10**6. All excess decimal points are rounded.
 * @param  amount               The amount to convert, e.g '10.5'
 * @return        A string representing the amount in microUnits e.g 10500000
 */
export function toMicroUnit(amount: Numeric.Input) {
  const dec = new Dec(amount);
  return dec.toDecimalPlaces(6).mul(10 ** 6).toFixed(0);
}

/**
 * The opposite of `toMicroUnit`
 * @param  amount               The amount to convert, e.g '1000000'
 * @return        A string representing the converted amount e.g '1.000000'
 */
export function fromMicroUnit(amount: Numeric.Input) {
  const int = new Int(amount);
  return int.dividedBy(10 ** 6).toFixed(6);
}

/**
 * Sends a transaction. This uses a *fixed* fee of 4500000uluna for more
 * predictable testing
 * @param  client               the LCDClient
 * @param  sender               the Wallet object of the sender
 * @param  msgs                 a Msg object or a list of Msg objects
 * @return the transaction result wrapped in a user-friendly TxResult class.
 * See txResult.ts for more details
 */
export async function sendTransaction(
  client: LCDClient,
  sender: Wallet,
  msgs: Msg[]|Msg,
) {
  const msgsFlat = [msgs].flat();
  const fixedFee = new Fee(30000000, [new Coin('uluna', 4500000)]);

  const tx = await sender.createAndSignTx({
    msgs: msgsFlat,
    fee: fixedFee,
    memo: 'Hello',
  });

  const txResult = await client.tx.broadcast(tx);

  if (isTxError(txResult)) {
    throw new Error(`Transaction failed! Here is the raw TX result object:\n${JSON.stringify(txResult)}`);
  }

  return new TxResult(txResult);
}

/**
 * Query the balance for a native token in microunits
 * e.g uluna, uust, ukrw etc
 * @param  client the LCDClient
 * @param  account the account to check. Can be either a string of the address
 * or a Wallet object
 * @param  denom the symbol of the token e.g. 'uluna' or 'uust'
 * @return         [description]
 */
export async function queryNativeTokenBalance(
  client: LCDClient,
  account: Wallet | string,
  denom: string = 'uusd',
) {
  let address: string;
  if (isWallet(account)) {
    address = account.key.accAddress;
  } else {
    address = account;
  }

  const [coins] = (await client.bank.balance(address));
  const coin = coins.get(denom);
  if (coin) {
    return coin.amount.toString();
  }
  return '0';
}

/**
 * Upload WASM bytecode to chain and receive a codeId back
 * @param  client the LCDClient
 * @param  deployer the wallet that uploads the code also referred to as the creator
 * @param  filepath path to WASM file
 * @return codeId as a number
 */
export async function storeCode(
  client: LCDClient,
  deployer: Wallet,
  filepath: string,
) {
  const bytecode = fs.readFileSync(filepath).toString('base64');
  const storeMsg = new MsgStoreCode(deployer.key.accAddress, bytecode);
  const txResult = await sendTransaction(client, deployer, storeMsg);
  return parseInt(txResult.getAttributeValue('store_code', 'code_id')[0], 10);
}

export async function instantiateContract(
  client: LCDClient,
  deployer: Wallet,
  codeId: number,
  initMsg: string | object,
  initCoins?: Coins.Input,
  admin?: string,
) {
  const instMsg = new MsgInstantiateContract(
    deployer.key.accAddress,
    admin,
    codeId,
    initMsg,
    initCoins,
  );
  const result = await sendTransaction(client, deployer, instMsg);
  return result.getAttributeValue('instantiate_contract', 'contract_address')[0];
}
