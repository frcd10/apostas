// Legacy hook no longer needed now that we call the
// on-chain program via raw web3.js instructions.
// Kept as a stub in case other components import it.
export function useBettingProgram() {
  return { program: null } as const;
}
