/* eslint-disable max-classes-per-file */
import path from 'path';

// TODO: Add address field
// TODO: Add more helper functions such as get owner etc etc
//       helpers should do minimal work and mostly use functions defined in utils.ts
// TODO: Get InitMsg (e.g how the contract was instantiated)
// TODO: Maybe add functionality to parse and print JSON expected format

export class CodeInfo {
  #data: CodeInfo.Data;

  constructor(codeId:string, wasmPath:string) {
    const data: CodeInfo.Data = { codeId, wasmPath: path.normalize(wasmPath) };
    this.#data = data;
  }

  /**
   * Return a friendly name identifying the smart contract. This is derived
   * from the filename containing the WASM bytecode with the extensions
   * stripped away.
   * @return a string as a friendly name for the smart contract
   */
  public get name() {
    return path.parse(path.normalize(this.#data.wasmPath)).name;
  }

  /**
   * Each WASM bytecode obtains a code ID when first uploaded onto the
   * blockchain. This code ID can be used to initialize multiple instances
   * of the contract, all sharing the same underlying logic.
   * @return a number representing the codeId
   */
  public get codeId() {
    return this.#data.codeId;
  }

  /**
   * @return the original path to the file containing the WASM bytecode
   */
  public get wasmPath() {
    return this.#data.wasmPath;
  }

  public toData() {
    return this.#data;
  }

  public static fromData(data: CodeInfo.Data) {
    return new this(data.codeId, data.wasmPath);
  }
}

/* eslint-disable-next-line no-redeclare */
export namespace CodeInfo {
  export interface Data {
    codeId: string,
    wasmPath: string
  }
}

export class Contract {
  #codeInfo: CodeInfo;

  constructor(codeInfo: CodeInfo) {
    this.#codeInfo = codeInfo;
  }

  /**
   * @return a CodeInfo object
   */
  public get codeInfo() {
    return this.#codeInfo;
  }
}
