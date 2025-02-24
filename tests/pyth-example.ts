import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PythExample } from "../target/types/pyth_example";
import { PublicKey } from "@solana/web3.js";

describe("pyth-example", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.PythExample as Program<PythExample>;

    it("Fetch price!", async () => {
        // Add your test here.
        const tx = await program.methods
            .sample()
            .accounts({ priceUpdate: new PublicKey("7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE") })
            .rpc();
        console.log("Your transaction signature", tx);
    });
});
