import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CnftVault } from "../target/types/cnft_vault";

describe("cnft-vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CnftVault as Program<CnftVault>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
