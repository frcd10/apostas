"use client";

import { FC, useState } from "react";
import { PublicKey, Connection } from "@solana/web3.js";
import { useWallet } from "@solana/wallet-adapter-react";
import toast from "react-hot-toast";
import { buildInitializeBetIx } from "../lib/bettingClient";

export const BetCreator: FC = () => {
  const wallet = useWallet();

  const [arbiter, setArbiter] = useState("");
  const [amountSol, setAmountSol] = useState("0.1");
  const [lockMinutes, setLockMinutes] = useState("5");
  const [isSubmitting, setIsSubmitting] = useState(false);

  const onSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!wallet.publicKey) {
      toast.error("Connect a wallet first");
      return;
    }

    try {
      setIsSubmitting(true);
      const arbiterPk = new PublicKey(arbiter);
      const lamports = Math.round(parseFloat(amountSol || "0") * 1_000_000_000);
      const lockSeconds = parseInt(lockMinutes || "0", 10) * 60;

      const ix = buildInitializeBetIx({
        creator: wallet.publicKey,
        arbiter: arbiterPk,
        betAmountLamports: lamports,
        lockSeconds,
      });

      const network = process.env.NEXT_PUBLIC_SOLANA_NETWORK || "devnet";
      const helius = process.env.NEXT_PUBLIC_HELIUS_RPC_URL;
      const fallback = process.env.NEXT_PUBLIC_RPC_URL || "https://api.devnet.solana.com";
      const rpc = helius || fallback;

      const connection = new Connection(rpc, "confirmed");

      const { Transaction } = await import("@solana/web3.js");
      const tx = new Transaction().add(ix);

      const sig = await wallet.sendTransaction(tx, connection);

      toast.success(`Bet created! Tx: ${sig.slice(0, 8)}...`);
    } catch (err: any) {
      console.error("create bet error", err);
      toast.error(err?.message || "Failed to create bet");
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <form onSubmit={onSubmit} className="space-y-4 rounded-xl border border-slate-700 bg-slate-900/60 p-4">
      <h2 className="text-lg font-semibold text-slate-50">Create a new bet</h2>
      <div className="space-y-1">
        <label className="block text-sm text-slate-300">Arbiter pubkey</label>
        <input
          type="text"
          required
          value={arbiter}
          onChange={(e) => setArbiter(e.target.value)}
          className="w-full rounded-md border border-slate-600 bg-slate-800 px-3 py-2 text-sm text-slate-50 focus:outline-none focus:ring-2 focus:ring-emerald-500"
          placeholder="Arbiter wallet address"
        />
      </div>
      <div className="grid grid-cols-2 gap-4">
        <div className="space-y-1">
          <label className="block text-sm text-slate-300">Bet amount (SOL)</label>
          <input
            type="number"
            min="0"
            step="0.01"
            value={amountSol}
            onChange={(e) => setAmountSol(e.target.value)}
            className="w-full rounded-md border border-slate-600 bg-slate-800 px-3 py-2 text-sm text-slate-50 focus:outline-none focus:ring-2 focus:ring-emerald-500"
          />
        </div>
        <div className="space-y-1">
          <label className="block text-sm text-slate-300">Lock period (minutes)</label>
          <input
            type="number"
            min="1"
            step="1"
            value={lockMinutes}
            onChange={(e) => setLockMinutes(e.target.value)}
            className="w-full rounded-md border border-slate-600 bg-slate-800 px-3 py-2 text-sm text-slate-50 focus:outline-none focus:ring-2 focus:ring-emerald-500"
          />
        </div>
      </div>
      <button
        type="submit"
        disabled={isSubmitting}
        className="inline-flex items-center justify-center rounded-md bg-emerald-500 px-4 py-2 text-sm font-medium text-white hover:bg-emerald-600 disabled:cursor-not-allowed disabled:opacity-60"
      >
        {isSubmitting ? "Creating bet..." : "Create bet"}
      </button>
      <p className="text-xs text-slate-400">
        This is a first draft wired to the on-chain program. Once the official IDL is available, we
        will replace the dummy IDL and extend this with group bets and advanced flows.
      </p>
    </form>
  );
};
