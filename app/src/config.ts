// Environment variable are injected during build time

// The rpc of the tezos node, by default, the rpc of a local tezos node is used
export const TEZOS_RPC = process.env.TEZOS_RPC || 'http://localhost:18731';

// The rpc of the rollup node, by default it's the local smart rollup node port
export const ROLLUP_RPC = process.env.ROLLUP_RPC || 'http://localhost:8932';
