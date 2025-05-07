import { Connection } from '@solana/web3.js';

export const getConnection = () => {
  return new Connection('https://rpc.lazorkit.xyz/', {
    wsEndpoint: 'https://rpc.lazorkit.xyz/ws/',
    commitment: 'confirmed',
    confirmTransactionInitialTimeout: 60000,
  });
};