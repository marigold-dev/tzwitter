// Environment variable are injected during build time

// The rpc of the tezos node, by default, the rpc of a local tezos node is used
// export const TEZOS_RPC = process.env.TEZOS_RPC || "http://localhost:18731";
export const TEZOS_RPC = 'https://rpc.dailynet-2023-03-24.teztnets.xyz';

// The rpc of the rollup node, by default it's the local smart rollup node port
export const ROLLUP_RPC = process.env.ROLLUP_RPC || 'http://localhost:8932';

// Commitment period
export const COMMITMENT_INTERVAL: number =
  Number(process.env.COMMITMENT_PERIOD) || 40; // Commitment interval

//
export const CEMENTED_PERIOD: number =
  Number(process.env.CEMENTED_PERIOD) || 40; // Time for a commit to be cemented

export const BLOCK_TIME: number = Number(process.env.BLOCK_TIME) || 30; // Time of a block
