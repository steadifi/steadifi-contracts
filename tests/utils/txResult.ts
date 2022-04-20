import { TxSuccess } from '@terra-money/terra.js/dist/client/lcd/api/TxAPI';

/**
 * Simple wrapper for result of successful transactions
 *
 * Provides a user-friendly method of getting attributes
 * @param txResultRaw  The txResult object returned from broadcasting the transaction
 */
class TxResult<B> {
  #txResultRaw: TxSuccess & B;

  constructor(txResultRaw: TxSuccess & B) {
    this.#txResultRaw = txResultRaw;
  }

  /**
   * Get the raw transaction result object
   * @return An item of type Block & TxSuccess
   */
  public getRawTxResult() {
    return this.#txResultRaw;
  }

  /**
   * A method to easily access the attributes of a transaction result.
   *
   * e.g res.getAttributes('coin_received', 'amount')
   * @param  eventType                   Type of event as a string
   * @param  attributeName               The key of the attribute
   * @return Return a list of values. Some attributes can appear twice which is
   * why the return type is a list
   */
  public getAttributes(eventType:string, attributeName:string) {
    return this.#txResultRaw.logs[0].eventsByType[eventType][attributeName];
  }
}

export default TxResult;
