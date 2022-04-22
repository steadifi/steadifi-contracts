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
   * e.g res.getAttributeValue('coin_received', 'amount')
   * @param  eventType                   Type of event as a string
   * @param  attributeName               The key of the attribute
   * @return Return a list of values. Some attributes can appear twice which is
   * why the return type is a list
   */
  public getAttributeValue(eventType:string, attributeName:string) {
    return this.#txResultRaw.logs[0].eventsByType[eventType][attributeName];
  }

  /**
   * @return a list of pairs [eventType, attributeName] of all possible valid
   * combinations of event types and attributes available to this transaction.
   * These can be used in a call to getAttributeValue(eventType, attributeName)
   */
  public getAllEventAttributes() {
    const eventTypes = Object.keys(this.#txResultRaw.logs[0].eventsByType);
    const pairs = eventTypes.flatMap((type) => (
      Object.keys(this.#txResultRaw.logs[0].eventsByType[type]).map((attribute) => (
        [type, attribute] as [string, string]
      ))));
    return pairs;
  }
}

export default TxResult;
