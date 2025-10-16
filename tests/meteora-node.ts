import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MeteoraNode } from "../target/types/meteora_node";
import { PublicKey } from "@solana/web3.js";

describe("meteora-node", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const SOLUSDC = new PublicKey("7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE");

  const program = anchor.workspace.MeteoraNode as Program<MeteoraNode>;

  it("Fetch Sol Price", async () => {
    const tx = await program.methods.fetchPrice("7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE").accountsPartial({
      signer: provider.wallet.publicKey,
      priceUpdateAccount: SOLUSDC,
    }).signers([provider.wallet.payer]).rpc();

    console.log(`Transaction Signature: ${tx}`);
  })
});