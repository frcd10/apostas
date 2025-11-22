import { useMemo } from 'react';
import { Connection } from '@solana/web3.js';

export function useWallet() {
  const connection = useMemo(
    () => new Connection(process.env.NEXT_PUBLIC_RPC_URL || 'https://api.devnet.solana.com'),
    []
  );

  return {
    connection,
  };
}
