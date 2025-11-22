import {
  PublicKey,
  SystemProgram,
  SYSVAR_CLOCK_PUBKEY,
  TransactionInstruction,
} from "@solana/web3.js";

// Must match the on-chain program id
export const BETTING_PROGRAM_ID = new PublicKey(
  process.env.NEXT_PUBLIC_PROGRAM_ID as string
);

export interface InitializeBetParams {
  creator: PublicKey;
  arbiter: PublicKey;
  betAmountLamports: number;
  lockSeconds: number;
}

// PDA helpers must mirror the seeds used in the Rust program
export function deriveBettingPoolPda(creator: PublicKey): PublicKey {
  const [pda] = PublicKey.findProgramAddressSync(
    [Buffer.from("bet_pool"), creator.toBuffer()],
    BETTING_PROGRAM_ID
  );
  return pda;
}

export function deriveVaultPda(bettingPool: PublicKey): PublicKey {
  const [pda] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), bettingPool.toBuffer()],
    BETTING_PROGRAM_ID
  );
  return pda;
}

// Build the initialize_bet instruction manually.
// This assumes instruction index 0 and a simple borsh-like layout:
// [variant: u8, arbiter_pk: [u8;32], bet_amount: u64, lock_seconds: u64]
export function buildInitializeBetIx(params: InitializeBetParams): TransactionInstruction {
  const { creator, arbiter, betAmountLamports, lockSeconds } = params;

  const bettingPool = deriveBettingPoolPda(creator);
  const vault = deriveVaultPda(bettingPool);

  const variant = 0; // initialize_bet

  const arbiterBytes = arbiter.toBytes();
  const amountBuffer = Buffer.alloc(8);
  amountBuffer.writeBigUInt64LE(BigInt(betAmountLamports));
  const lockBuffer = Buffer.alloc(8);
  lockBuffer.writeBigUInt64LE(BigInt(lockSeconds));

  const data = Buffer.concat([
    Buffer.from([variant]),
    Buffer.from(arbiterBytes),
    amountBuffer,
    lockBuffer,
  ]);

  const keys = [
    { pubkey: creator, isSigner: true, isWritable: true },
    { pubkey: arbiter, isSigner: false, isWritable: false },
    { pubkey: bettingPool, isSigner: false, isWritable: true },
    { pubkey: vault, isSigner: false, isWritable: true },
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    { pubkey: SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: false },
  ];

  return new TransactionInstruction({
    programId: BETTING_PROGRAM_ID,
    keys,
    data,
  });
}
