import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaDex } from "../target/types/solana_dex";
import { assert } from "chai";

describe("solana-dex", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaDex as Program<SolanaDex>;

  it("Is initialized!", async () => {
    // Add your test here.
  });

  it("Fail when add liquidity insufficient funds", async () => {});

  it("Add liquidity successful", async () => {});

  it("Fail when swap token insufficient funds", async () => {});

  it("Swap token successful", async () => {});
});
