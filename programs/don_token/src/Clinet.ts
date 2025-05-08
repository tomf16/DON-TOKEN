import * as anchor from "@coral-xyz/anchor";
import { SystemProgram } from "@solana/web3.js";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

(async () => {
  const program = anchor.workspace.DonToken;
  const tokenState = anchor.web3.Keypair.generate();

  const marketingWallet = new anchor.web3.PublicKey("7j5igm2vApexsX6AUYgC4TPsw62MaQQZq6QLCZNHmgK8");
  const liquidityWallet = new anchor.web3.PublicKey("4666x6whrFgwr1NMedF8CFLnU3nXXyJVMiPzvpLcdEc7");
  const burnWallet = new anchor.web3.PublicKey("11111111111111111111111111111111");
  const teamWallet = new anchor.web3.PublicKey("396WkQzwyQb5VtGFrgJwCAVmdKtRBrvRiLU5nPgLLAw6");
  const reserveWallet = new anchor.web3.PublicKey("89MLgfjjde4DJebpKgAmJC5jLZLsUwdYG3FZwMWr1Fhe");

  const space = 8 + 32 * 7 + 8 + (4 + 50 * 32);
  const lamports = await provider.connection.getMinimumBalanceForRentExemption(space);

  const createIx = anchor.web3.SystemProgram.createAccount({
    fromPubkey: provider.wallet.publicKey,
    newAccountPubkey: tokenState.publicKey,
    space,
    lamports,
    programId: program.programId,
  });

  const createTx = new anchor.web3.Transaction().add(createIx);
  await provider.sendAndConfirm(createTx, [tokenState]);

  const initTx = await program.methods
    .initialize(new anchor.BN(1_000_000_000))
    .accounts({
      owner: provider.wallet.publicKey,
      tokenState: tokenState.publicKey,
      marketingWallet,
      liquidityWallet,
      burnWallet,
      teamWallet,
      reserveWallet,
      systemProgram: SystemProgram.programId,
    })
    .signers([tokenState])
    .rpc();

  console.log("Initialize successful");
  console.log("TokenState Address:", tokenState.publicKey.toBase58());
  console.log("Transaction Signature:", initTx);

  const receiver = anchor.web3.Keypair.generate();

  const transferTx = await program.methods
    .transfer(new anchor.BN(1_000_000))
    .accounts({
      tokenState: tokenState.publicKey,
      sender: provider.wallet.publicKey,
      receiver: receiver.publicKey,
      marketingWallet,
      liquidityWallet,
      burnWallet,
    })
    .rpc();

  console.log("Transfer successful");
  console.log("Receiver:", receiver.publicKey.toBase58());
  console.log("Transfer Tx Signature:", transferTx);
})();
