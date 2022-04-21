import fs from 'fs';
import { LCDClient } from '@terra-money/terra.js';

import { createLCDClient, getTestWallet } from './utils';
import { TestAccountName } from './testAccounts';
import { CodeInfo } from './contract';

class Context {
  static #filename: string = './.tmp_context';

  static #instance: Context;

  #client: LCDClient;

  #data: Context.Data;

  public get client() {
    return this.#client;
  }

  public getTestWallet(name: TestAccountName) {
    return getTestWallet(this.client, name);
  }

  public addCodeInfo(codeId:string, wasmPath:string) {
    const codeInfo = new CodeInfo(codeId, wasmPath);
    this.#data.codes[codeInfo.name] = codeInfo.toData();
  }

  public getCodeInfo(name: string) {
    return CodeInfo.fromData(this.#data.codes[name]);
  }

  public toFile() {
    fs.writeFileSync(Context.#filename, JSON.stringify(this.#data, null, 2), 'utf8');
  }

  private constructor() {
    this.#client = createLCDClient();
    this.#data = { codes: {} };
  }

  public static fromFile() {
    const ctx = Context.instance();
    const data = JSON.parse(fs.readFileSync(this.#filename, 'utf8'));
    Object.assign(ctx.#data, data);
    return ctx;
  }

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
  }
}

export default Context;
