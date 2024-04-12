import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaDex } from "../target/types/solana_dex"; // Adjust with your actual target types
import { assert } from "chai";
import {
  TOKEN_PROGRAM_ID,
  MintLayout,
  AccountLayout,
  createMint,
} from "@solana/spl-token";

describe("solana-dex", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SolanaDex as Program<SolanaDex>;

  let mint = null;
  let depositorTokenAccount = null;
  let pairTokenAccount = null;
  let depositor = anchor.web3.Keypair.generate();

  before(async () => {
    // Create the mint for the MOVE token
    mint = await createMint(
      provider.connection,
      anchor.web3.Keypair.generate(),
      provider.wallet.publicKey,
      null,
      9,
      anchor.web3.Keypair.generate()
    );

    // Create token accounts for depositor and the pair
    depositorTokenAccount = await mint.createAccount(depositor.publicKey);
    pairTokenAccount = await mint.createAccount(provider.wallet.publicKey);

    // Mint some tokens to the depositor account
    await mint.mintTo(
      depositorTokenAccount,
      provider.wallet.publicKey,
      [],
      1000
    );
  });

  it("Add liquidity successfully", async () => {
    const amount = new anchor.BN(500); // Amount of tokens to transfer for liquidity

    // Simulate add liquidity
    await program.methods
      .addLiquidity(amount)
      .accounts({
        pairTokenAccount: pairTokenAccount,
        pairMetadata: "Your pair metadata account", // This needs to be correctly initialized
        tokenMintAddress: mint.publicKey,
        authority: provider.wallet.publicKey,
        depositorTokenAccount: depositorTokenAccount,
        depositor: depositor.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([depositor]) // Signer must be the depositor if required
      .rpc();

    // Fetch the updated balance of the pair token account
    const balance = await mint.getAccountInfo(pairTokenAccount);
    assert.equal(
      balance.amount.toString(),
      "500",
      "The liquidity wasn't added correctly"
    );
  });

  it("Fail to add liquidity due to insufficient funds", async () => {
    const amount = new anchor.BN(1500); // Attempt to transfer more than the depositor's balance

    try {
      await program.methods
        .addLiquidity(amount)
        .accounts({
          pairTokenAccount: pairTokenAccount,
          pairMetadata: "Your pair metadata account", // Ensure proper setup
          tokenMintAddress: mint.publicKey,
          authority: provider.wallet.publicKey,
          depositorTokenAccount: depositorTokenAccount,
          depositor: depositor.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([depositor])
        .rpc();

      assert.fail("The transaction should have failed but did not.");
    } catch (err) {
      // Check for the specific error; make sure it's the one you throw for insufficient funds
      assert.include(
        err.message,
        "insufficient funds",
        "The error message should indicate insufficient funds"
      );
    }
  });
});
