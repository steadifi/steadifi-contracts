import path from 'path';

// TODO: Add address field
// TODO: Add more helper functions such as get owner etc etc
//       helpers should do minimal work and mostly use functions defined in utils.ts
// TODO: Get InitMsg (e.g how the contract was instantiated)
// TODO: Maybe add functionality to parse and print JSON expected format

class Contract {
  #codeId: number;

  #wasmPath: path.ParsedPath;

  constructor(codeId:number, wasmPath:string) {
    this.#codeId = codeId;
    this.#wasmPath = path.parse(path.normalize(wasmPath));
  }

  /**
   * Return a friendly name identifying the smart contract. This is derived
   * from the filename containing the WASM bytecode with the extensions
   * stripped away.
   * @return a string as a friendly name for the smart contract
   */
  public get name() {
    return this.#wasmPath.name;
  }

  /**
   * Each WASM bytecode obtains a code ID when first uploaded onto the
   * blockchain. This code ID can be used to initialize multiple instances
   * of the contract, all sharing the same underlying logic.
   * @return a number representing the codeId of the contract
   */
  public get codeId() {
    return this.#codeId;
  }

  /**
   * @return the original path to the file containing the WASM bytecode
   */
  public get wasmPath() {
    return this.#wasmPath;
  }
}

export default Contract;
