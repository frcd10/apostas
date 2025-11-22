"use client";

import React from "react";
import { Toaster } from "react-hot-toast";
import { WalletConnect } from "../components/WalletConnect";
import { BetCreator } from "../components/BetCreator";

const Home: React.FC = () => {
  return (
    <main className="min-h-screen bg-gradient-to-b from-slate-950 via-slate-900 to-black text-slate-50">
      <Toaster position="top-right" />
      <div className="mx-auto flex max-w-5xl flex-col gap-10 px-4 py-10">
        <header className="flex flex-col gap-6 rounded-2xl border border-emerald-500/30 bg-slate-950/60 p-6 shadow-[0_0_60px_rgba(16,185,129,0.3)]">
          <div className="space-y-2">
            <p className="text-xs uppercase tracking-[0.25em] text-emerald-400">degen arena</p>
            <h1 className="text-4xl font-extrabold tracking-tight">
              Bet or get <span className="text-emerald-400">rekt</span>.
            </h1>
            <p className="max-w-2xl text-sm text-slate-300">
              Spin up trust-minimized head–to–head bets with an arbiter, time locks and group action.
              All on Solana devnet.
            </p>
          </div>
          <div className="grid gap-4 text-xs text-slate-300 sm:grid-cols-3">
            <div className="rounded-lg border border-slate-800 bg-slate-900/60 p-3">
              <p className="font-semibold text-slate-50">On-chain vaults</p>
              <p className="mt-1 text-slate-400">SOL locked into PDAs until arbiter calls it.</p>
            </div>
            <div className="rounded-lg border border-slate-800 bg-slate-900/60 p-3">
              <p className="font-semibold text-slate-50">Arbiter fees</p>
              <p className="mt-1 text-slate-400">Basis-points fees to keep the ref well fed.</p>
            </div>
            <div className="rounded-lg border border-slate-800 bg-slate-900/60 p-3">
              <p className="font-semibold text-slate-50">Group action</p>
              <p className="mt-1 text-slate-400">Side pots and crowd bets coming next.</p>
            </div>
          </div>
        </header>

        <section className="grid gap-8 md:grid-cols-[1.6fr_1.1fr]">
          <div className="space-y-4">
            <WalletConnect>
              <BetCreator />
            </WalletConnect>
          </div>

          <aside className="space-y-4 rounded-2xl border border-slate-800 bg-slate-950/70 p-4 text-sm text-slate-300">
            <p className="text-xs font-semibold uppercase tracking-[0.2em] text-slate-400">
              session feed
            </p>
            <div className="space-y-2 text-xs">
              <p className="text-slate-400">
                • Connect with Phantom / Solflare.
              </p>
              <p className="text-slate-400">
                • Drop an arbiter address you trust.
              </p>
              <p className="text-slate-400">
                • Pick your size in SOL &amp; lock window.
              </p>
              <p className="text-slate-500">
                • Next: live odds, group pools and on-chain history.
              </p>
            </div>
          </aside>
        </section>
      </div>
    </main>
  );
};

export default Home;
