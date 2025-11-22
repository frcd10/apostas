"use client";

import { FC, useMemo, useEffect, useState } from "react";
import { ConnectionProvider, WalletProvider, useWallet } from "@solana/wallet-adapter-react";
import { WalletModalProvider, WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { PhantomWalletAdapter, SolflareWalletAdapter } from "@solana/wallet-adapter-wallets";
import "@solana/wallet-adapter-react-ui/styles.css";

const InnerWalletButton: FC = () => {
  const wallet = useWallet();
  return (
    <div className="flex items-center justify-between gap-4 rounded-xl border border-slate-700 bg-slate-900/60 px-4 py-3">
      <div className="text-sm text-slate-300">
        {wallet.connected && wallet.publicKey
          ? `Connected: ${wallet.publicKey.toBase58().slice(0, 4)}...${wallet.publicKey
              .toBase58()
              .slice(-4)}`
          : "Connect your Solana wallet to start betting"}
      </div>
      <WalletMultiButton className="btn btn-primary" />
    </div>
  );
};

export const WalletConnect: FC<{ children?: React.ReactNode }> = ({ children }) => {
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  const network = process.env.NEXT_PUBLIC_SOLANA_NETWORK || "devnet";
  const helius = process.env.NEXT_PUBLIC_HELIUS_RPC_URL;
  const fallback = process.env.NEXT_PUBLIC_RPC_URL || "https://api.devnet.solana.com";
  const endpoint = helius || fallback;

  const wallets = useMemo(
    () => [new PhantomWalletAdapter(), new SolflareWalletAdapter({ network: network as any })],
    [network]
  );

  if (!mounted) {
    return null;
  }

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <div className="space-y-6">
            <InnerWalletButton />
            {children}
          </div>
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
};
