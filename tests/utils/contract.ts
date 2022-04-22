/* eslint-disable max-classes-per-file */
import path from 'path';

export class CodeInfo {
  #data: CodeInfo.Data;

  constructor(codeId:number, wasmPath:string) {
    const data: CodeInfo.Data = { codeId, wasmPath: path.resolve(path.normalize(wasmPath)) };
    this.#data = data;
  }

  /**
   * Return a friendly name identifying the smart contract. This is derived
   * from the filename containing the WASM bytecode with the extensions
   * stripped away.
   * @return a string as a friendly name for the smart contract
   */
  public get name() {
    return path.parse(path.resolve(path.normalize(this.#data.wasmPath))).name;
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
    codeId: number,
    wasmPath: string
  }
}

export class ContractInfo {
  #data: ContractInfo.Data;

  constructor(identifier: string, codeId: number, contractAddress: string) {
    this.#data = { identifier, codeId, contractAddress };
  }

  /**
   * @return the codeId used by this contract
   */
  public get identifier() {
    return this.#data.identifier;
  }

  /**
   * @return the codeId used by this contract
   */
  public get codeId() {
    return this.#data.codeId;
  }

  /**
   * @return the codeId used by this contract
   */
  public get contractAddress() {
    return this.#data.contractAddress;
  }

  public toData() {
    return this.#data;
  }

  public static fromData(data: ContractInfo.Data) {
    return new this(data.identifier, data.codeId, data.contractAddress);
  }
}

/* eslint-disable-next-line no-redeclare */
export namespace ContractInfo {
  export interface Data {
    identifier: string,
    codeId: number,
    contractAddress: string,
  }
}
