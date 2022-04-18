import fs from 'fs';
import { LCDClient } from '@terra-money/terra.js';

import { createLCDClient } from './utils';

class Context {
  static #filename: string = './.tmp_context';

  static #instance: Context;

  #client: LCDClient;

  public get client() {
    return this.#client;
  }

  private constructor() {
    this.#client = createLCDClient();
  }

  public toFile() {
    const data = {};
    fs.writeFileSync(Context.#filename, JSON.stringify(data), 'utf8');
  }

  public static fromFile() {
    const ctx = Context.instance();
    const data = JSON.parse(fs.readFileSync(this.#filename, 'utf8'));
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
