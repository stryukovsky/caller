import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Caller } from "../target/types/caller";
import { Callee } from "../target/types/callee";
import { expect } from "chai";

describe("caller", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const caller = anchor.workspace.Caller as Program<Caller>;
  const callee = anchor.workspace.Callee as Program<Callee>;

  const clockData = anchor.web3.Keypair.generate();

  it("should initialize callee", async () => {
    await callee.methods.initialize().accounts({
        clockData: clockData.publicKey
    }).signers([clockData]).rpc();
  });

  it("should set clock value from caller", async () => {
    await caller.methods.call().accounts({
        calleeProgram: callee.programId,
        clockData: clockData.publicKey
    }).rpc();
    const timestamp = (await callee.account.clockData.fetch(clockData.publicKey)).timestamp.toNumber();
    const actualTime = Date.now() / 1000;
    expect(timestamp).approximately(actualTime, 5);
  });
});
