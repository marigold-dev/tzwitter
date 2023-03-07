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

type OptionsOpt = {
  verbose?: boolean;
};

type Options = {
  verbose: boolean;
};

/**
 * Bette log function to log http request
 * @param url the url
 * @param status the response tatus
 * @param body the body (optionnal)
 */
const log = (url: string, status: number, body?: unknown) => {
  console.groupCollapsed(url);
  if (body) console.log(body);
  console.log(`Status: ${status}`);
  console.groupEnd();
};

/**
 * Retrieves the options
 */
const getOptions = (options?: OptionsOpt): Options => {
  const defaultValues = {
    verbose: false,
  };
  if (!options) return defaultValues;
  return { ...defaultValues, ...options };
};

const get = async (url: string, opt?: OptionsOpt) => {
  const { verbose } = getOptions(opt);
  const res = await fetch(url);
  if (verbose) log(url, res.status);
  if (!res.ok) {
    console.error(`${url} returns ${res.status}`);
    throw new Error('Not a 200');
  }
  return res.json();
};

const post = async (url: string, body: unknown, opt?: OptionsOpt) => {
  const { verbose } = getOptions(opt);
  const headers = new Headers();
  headers.append('Content-type', 'application/json');
  const options = {
    method: 'POST',
    body: JSON.stringify(body),
    headers,
  };
  const res = await fetch(url, options);
  if (verbose) log(url, res.status, body);
  if (!res.ok) {
    console.error(`${url} returns ${res.status}`);
    throw new Error('Not a 200');
  }
  return res.json();
};

/**
 * Client to interact with a rollup
 */
class RollupClient {
  private tezosUrl: string;
  private rollupUrl: string;
  private signer: Signer;
  private verbose?: boolean;

  constructor({
    signer,
    tezosUrl,
    rollupUrl,
    verbose,
  }: {
    signer: Signer;
    tezosUrl: string;
    rollupUrl: string;
    verbose?: boolean;
  }) {
    this.tezosUrl = tezosUrl;
    this.rollupUrl = rollupUrl;
    this.signer = signer;
    this.verbose = verbose;
  }

  /**
   * Sends a payload to the rollup inbox
   * It will simulate the operation
   * @param payload some bytes to send to the rollup shared inbox
   * @returns the hash of the operation
   */
  async send(payload: string): Promise<string> {
    const options = { verbose: this.verbose };
    const tezosUrl = this.tezosUrl;

    // retrieve the counter of the address
    const previousCounter = await get(
      `${tezosUrl}/chains/main/blocks/head/context/contracts/tz1QFD9WqLWZmmAuqnnTPPUjfauitYEWdshv/counter`,
      options,
    );
    const counter = Number.parseInt(previousCounter) + 1; // We direct increment the counter

    // The hash of head~2
    const previousHashForSimulation = await get(
      `${tezosUrl}/chains/main/blocks/head~2/hash`,
      options,
    );

    // chain id
    const chainId = await get(`${tezosUrl}/chains/main/chain_id`, options);

    // Simulate the operation
    const operationToSimulate = {
      operation: {
        branch: previousHashForSimulation,
        contents: [
          {
            kind: 'smart_rollup_add_messages',
            source: 'tz1QFD9WqLWZmmAuqnnTPPUjfauitYEWdshv',
            fee: '0',
            counter: counter.toString(),
            gas_limit: '100000',
            storage_limit: '0',
            message: [payload],
          },
        ],
      },
      chain_id: chainId,
    };
    const simulatedOperation = await post(
      `${tezosUrl}/chains/main/blocks/head/helpers/scripts/simulate_operation`,
      operationToSimulate,
      options,
    );
    // Retrieve the needed gas
    const estimatedGas =
      simulatedOperation.contents[0].metadata['operation_result'][
        'consumed_milligas'
      ];

    // The hash of head~2
    const previousHash = await get(
      `${tezosUrl}/chains/main/blocks/head~2/hash`,
      options,
    );

    // Forge the operation
    const operationToForge = {
      branch: previousHash,
      contents: [
        {
          kind: 'smart_rollup_add_messages',
          source: 'tz1QFD9WqLWZmmAuqnnTPPUjfauitYEWdshv',
          fee: estimatedGas, // TODO: find the minimal fee
          counter: counter.toString(),
          gas_limit: estimatedGas,
          storage_limit: '100', // TODO: find the minitmal storage limit
          message: [payload],
        },
      ],
    };

    const forgedOperationResult = await post(
      `${tezosUrl}/chains/main/blocks/head/helpers/forge/operations`,
      operationToForge,
      options,
    );
    const forgedOperation = '03' + forgedOperationResult; // prefix with 03 because it's a Tezos operation

    // Sign the bytes
    const { sbytes } = await this.signer.sign(forgedOperation);

    // Inject the operation
    const toInject = sbytes.slice(2); // remove the 03 bytes when inject
    const opHash = await post(
      `${tezosUrl}/injection/operation?chain=main`,
      toInject,
      options,
    );
    // returns the operation hash
    return opHash;
  }

  /**
   * Get the state of the value of the state at a given path
   * @param path
   * @returns
   */
  async getState(path: string) {
    const options = { verbose: this.verbose };
    const rollupUrl = this.rollupUrl;
    const res = await get(
      `${rollupUrl}/global/block/head/durable/wasm_2_0_0/value?key=${path}`,
      options,
    );
    return res;
  }

  /**
   * Get the list of keys for a given key
   * @param path
   * @returns
   */
  async getSubkeys(path: string) {
    const options = { verbose: this.verbose };
    const rollupUrl = this.rollupUrl;
    const res = await get(
      `${rollupUrl}/global/block/head/durable/wasm_2_0_0/subkeys?key=${path}`,
      options,
    );
    return res;
  }
}

export { RollupClient, type Signer };
