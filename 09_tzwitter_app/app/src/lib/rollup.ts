import { TezosToolkit } from '@taquito/taquito';
import { SmartRollupAddMessagesOperation } from '@taquito/taquito/dist/types/operations/smart-rollup-add-messages-operation';

// This client has to be generic, it should adapt to any rollups

interface Signer {
  sign: (bytes: string) => Promise<{
    bytes: string;
    sig: string;
    prefixSig: string;
    sbytes: string;
  }>;
  publicKey: () => Promise<string>;
  publicKeyHash: () => Promise<string>;
}

/**
 * Client to interact with a rollup
 * It's wrapped arround Taquito
 * And it adds two new methods:
 *   - getState
 *   - getSubKeys
 */
class RollupClient {
  private tezos: TezosToolkit;
  private rollupUrl: string;

  constructor({
    tezos,
    rollupUrl,
  }: {
    tezos: TezosToolkit;
    rollupUrl: string;
  }) {
    this.tezos = tezos;
    this.rollupUrl = rollupUrl;
  }

  /**
   * Sends a payload to the rollup inbox
   * @param payload some bytes to send to the rollup shared inbox
   * @returns smart rollup message operation
   */
  async send(payload: string): Promise<SmartRollupAddMessagesOperation> {
    const op = await this.tezos.contract.smartRollupAddMessages({
      message: [payload],
    });
    return op;
  }

  /**
   * Get the state of the value of the state at a given path
   * @param path
   * @returns
   */
  async getState(path: string) {
    const rollupUrl = this.rollupUrl;
    const url = `${rollupUrl}/global/block/head/durable/wasm_2_0_0/value?key=${path}`;
    const res = await fetch(url);
    if (!res.ok) {
      console.error(`${url} returns ${res.status}`);
    }
    return res.json();
  }

  /**
   * Get the list of keys for a given key
   * @param path
   * @returns
   */
  async getSubkeys(path: string) {
    const rollupUrl = this.rollupUrl;
    const url = `${rollupUrl}/global/block/head/durable/wasm_2_0_0/subkeys?key=${path}`;
    const res = await fetch(url);
    if (!res.ok) {
      console.error(`${url} returns ${res.status}`);
    }
    return res.json();
  }
}

export { RollupClient, type Signer };
