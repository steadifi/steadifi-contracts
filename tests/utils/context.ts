import fs from 'fs';
import { LCDClient } from '@terra-money/terra.js';

import { createLCDClient, getTestWallet } from './utils';
import { TestAccountName } from './testAccounts';
import { CodeInfo, ContractInfo } from './contract';

/**
 * This is a singleton class. The instance will be shared by all tests in a
 * test suite. It holds information that are useful during testing such as
 * codeIds or instantiated contracts
 */
class Context {
  static #filename: string = './.tmp_context';

  static #instance: Context;

  #client: LCDClient;

  #data: Context.Data;

  /**
   * Get the currently active LCDClient
   * @return an LCDClient
   */
  public get client() {
    return this.#client;
  }

  /**
   * Get a test wallet. In LocalTerra, these wallets are initialized with 10 LUNA
   * @param  name a short name for the wallet e.g validator, test1, test2, etc
   * @return a Wallet instance
   */
  public getTestWallet(name: TestAccountName) {
    return getTestWallet(this.client, name);
  }

  /**
   * Add code id and the WASM filepath to the context. Then this information
   * will become available during testing
   * @param  codeId code id of uploaded code
   * @param  wasmPath path to the wasm file. This is used to derive a friendly
   * name for the code which can then be used to access the code id at a
   * later time.
   */
  public addCodeInfo(codeId:number, wasmPath:string) {
    const codeInfo = new CodeInfo(codeId, wasmPath);

    if (codeInfo.name in this.#data.codes) {
      const existingEntry = this.getCodeInfo(codeInfo.name);
      if (existingEntry.codeId !== codeInfo.codeId
        || existingEntry.wasmPath !== codeInfo.wasmPath) {
        throw new Error(`An entry with contract name ${codeInfo.name} already exists.`);
      } else {
        return codeInfo;
      }
    }

    this.#data.codes[codeInfo.name] = codeInfo.toData();
    return codeInfo;
  }

  /**
   * Get a CodeInfo object using just the name of the code.
   * e.g oracle_manager
   * @param  name a friendly identifier for the code
   * @return a CodeInfo object
   */
  public getCodeInfo(name: string) {
    return CodeInfo.fromData(this.#data.codes[name]);
  }

  /**
   * Add instantiated contract info to the context. Then this information
   * will become available during testing.
   * @param  contractName Name of the contract/code. This name needs to match
   * the one given during the upload of the code
   * @param  contractAddress The address of this instantiated contract
   * @param  suffix the suffix to append to the contractName, which will be used
   * as a unique identifier for this particular instantiation. If a suffix is not
   * provided, a unique one will be generated automatically. The addContractInfo
   * returns the final ContractInfo object, therefore you can simply retrieve
   * the final identifier from that object
   */
  public addContractInfo(
    contractName: string,
    contractAddress:string,
    suffix?:string,
  ) {
    // Check existing data for consistency
    if (!(contractName in this.#data.codes)) {
      throw new Error(`Could not find contract with name ${contractName} in this context instance.`);
    }

    const { codeId } = this.getCodeInfo(contractName);

    // Find unique suffix automatically if one is not given
    let suffixFinal = suffix;
    if (suffixFinal === undefined) {
      for (let i = 0; i <= Object.keys(this.#data.contracts).length; i++) {
        const identifier = `${contractName}_${i}`;
        if (!(identifier in this.#data.contracts)) {
          suffixFinal = `_${i}`;
          break;
        }
      }
    }

    const identifier = contractName + (suffixFinal as string);
    const contractInfo = new ContractInfo(identifier, codeId, contractAddress);
    this.#data.contracts[contractInfo.identifier] = contractInfo.toData();
    return contractInfo;
  }

  /**
   * Get a ContractInfo object using just the identifier of the contract.
   * e.g oracle_manager_1
   * @param  identifier a friendly identifier for the contract
   * @return a ContractInfo object
   */
  public getContractInfo(identifier: string) {
    return ContractInfo.fromData(this.#data.contracts[identifier]);
  }

  /**
   * Store state of the current Context object to a file
   */
  public toFile() {
    fs.writeFileSync(Context.#filename, JSON.stringify(this.#data, null, 2), 'utf8');
  }

  private constructor() {
    this.#client = createLCDClient();
    this.#data = { codes: {}, contracts: {} };
  }

  /**
   * Fetch the current Context instance (or create a new one) and load its
   * state from file
   * @return Context instance
   */
  public static fromFile() {
    const ctx = Context.instance();
    const data = JSON.parse(fs.readFileSync(this.#filename, 'utf8'));
    Object.assign(ctx.#data, data);
    return ctx;
  }

  /**
   * Get the current Context instance or create a new one
   * @return Context instance
   */
  public static instance() {
    if (!this.#instance) {
      this.#instance = new this();
    }
    return this.#instance;
  }
}

/* eslint-disable-next-line no-redeclare */
namespace Context {
  export interface Data {
    codes: {[name:string] : CodeInfo.Data};
    contracts: {[identifier:string] : ContractInfo.Data};
  }
}

export default Context;
