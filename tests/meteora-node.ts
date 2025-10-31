import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { Dlmm } from "../target/types/dlmm";
import { PublicKey, Keypair } from "@solana/web3.js";
import DLMM from "@meteora-ag/dlmm";

describe("meteora-node", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const SOLUSD = new PublicKey("7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE");

  const program = anchor.workspace.Dlmm as Program<Dlmm>;

  it("Fetch Sol Price", async () => {
    const tx = await program.methods.fetchPrice().accountsPartial({
      signer: provider.wallet.publicKey,
      priceUpdateAccount: SOLUSD,
    }).signers([provider.wallet.payer]).rpc();

    console.log(`Transaction Signature: ${tx}`);
  });

  it("Add Liquidity By Strategy", async () => {
    const DLMM_POOL_ADDRESS = new PublicKey("ARwi1S4DaiTG5DX7S4M4ZsrXqpMD1MrTmbu9ue2tpmEq");

    console.log("Creating DLMM pool instance...");
    console.log("Network:", provider.connection.rpcEndpoint);

    let dlmmPool: DLMM;
    try {
      dlmmPool = await DLMM.create(provider.connection, DLMM_POOL_ADDRESS);
    } catch (error) {
      console.error("\nError details:", error.message);
      throw error;
    }

    // Get active bin information
    const activeBin = await dlmmPool.getActiveBin();
    console.log("Active Bin ID:", activeBin.binId);
    console.log("Active Bin Price (lamports):", activeBin.price);
    console.log("Active Bin Price (per token):", dlmmPool.fromPricePerLamport(Number(activeBin.price)));

    const feeInfo = dlmmPool.getFeeInfo();

    const TOTAL_RANGE_INTERVAL = 10;
    const minBinId = activeBin.binId - TOTAL_RANGE_INTERVAL;
    const maxBinId = activeBin.binId + TOTAL_RANGE_INTERVAL;

    const totalXAmount = new BN(100_000);
    const totalYAmount = new BN(100_000);
    const newPosition = Keypair.generate();

    const userTokenX = dlmmPool.tokenX.publicKey;
    const userTokenY = dlmmPool.tokenY.publicKey;
    const reserveX = dlmmPool.lbPair.reserveX;
    const reserveY = dlmmPool.lbPair.reserveY;

    const [eventAuthority] = PublicKey.findProgramAddressSync(
      [Buffer.from("__event_authority")],
      new PublicKey("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo")
    );


    // Define strategy type
    // Available variants: spotOneSide, curveOneSide, bidAskOneSide,
    // spotBalanced, curveBalanced, bidAskBalanced,
    // spotImBalanced, curveImBalanced, bidAskImBalanced
    const strategyType = { spotBalanced: {} }; // Using Spot Balanced strategy
    const maxSlippage = 300; // 3% slippage (300 basis points = 3%)

    try {
      const tx = await program.methods
        .addLiquidityByStrategy(
          totalXAmount,
          totalYAmount,
          minBinId,
          maxBinId,
          strategyType,
          activeBin.binId,
          maxSlippage
        )
        .accountsPartial({
          position: newPosition.publicKey,
          lbPair: dlmmPool.pubkey,
          binArrayBitmapExtension: null, // Optional account
          userTokenX: userTokenX,
          userTokenY: userTokenY,
          reserveX: reserveX,
          reserveY: reserveY,
          tokenXMint: dlmmPool.tokenX.publicKey,
          tokenYMint: dlmmPool.tokenY.publicKey,
          sender: provider.wallet.publicKey,
          tokenXProgram: new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
          tokenYProgram: new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
          eventAuthority: eventAuthority,
          lbClmmProgram: new PublicKey("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([provider.wallet.payer, newPosition])
        .rpc();

      const { userPositions } = await dlmmPool.getPositionsByUserAndLbPair(
        provider.wallet.publicKey
      );

      const createdPosition = userPositions.find(pos =>
        pos.publicKey.equals(newPosition.publicKey)
      );

      if (createdPosition) {
        console.log("Position found!");
        console.log("Position Bins:", createdPosition.positionData.positionBinData.length);
        console.log("Bin Data:");
        createdPosition.positionData.positionBinData.forEach((bin, idx) => {
          console.log(`  Bin ${idx + 1}: ID=${bin.binId}, Liquidity Shares=${bin.positionXAmount.toString()}`);
        });
      } else {
        console.log("Position not found (may take a moment to index)");
      }

    } catch (error) {
      console.error("Error:", error);
      throw error;
    }
  });
});
