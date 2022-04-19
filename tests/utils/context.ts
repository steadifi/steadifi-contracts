import fs from 'fs';
import { LCDClient } from '@terra-money/terra.js';

import { createLCDClient } from './utils';
import getMnemonicKey from './testAccounts';

class Context {
  static #filename: string = './.tmp_context';

  static #instance: Context;

  #client: LCDClient;

  public get client() {
    return this.#client;
  }

  public getTestWallet(name: Parameters<typeof getMnemonicKey>[0]) {
    return this.client.wallet(getMnemonicKey(name));
  }

  public toFile() {
    fs.writeFileSync(Context.#filename, JSON.stringify(this), 'utf8');
  }

  private constructor() {
    this.#client = createLCDClient();
  }

  public static fromFile() {
    const ctx = Context.instance();
    const data = JSON.parse(fs.readFileSync(this.#filename, 'utf8'));
    Object.assign(ctx, data);
    return ctx;
  }

  public static instance() {
    if (!this.#instance) {
      this.#instance = new this();
    }
    return this.#instance;
  }
}

export default Context;
