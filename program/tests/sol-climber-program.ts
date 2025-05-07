import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { SolClimberProgram } from "../target/types/sol_climber_program";
import { ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { SYSVAR_RENT_PUBKEY } from "@solana/web3.js";

describe("sol-climber-program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.solClimberProgram as Program<SolClimberProgram>;
  const payer = provider.wallet;
  // const player = anchor.web3.Keypair.generate();

  // Metaplex Constants
  const METADATA_SEED = "metadata";
  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

  const [playerPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("player"), payer.publicKey.toBuffer()],
    program.programId,
  );

  const [inventoryPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("inventory"), payer.publicKey.toBuffer()],
    program.programId,
  );

  it("Initialize Player", async () => {
    // await airdrop(player.publicKey);

    await program.methods
      .initializePlayer()
      .accountsPartial({
        signer: payer.publicKey,
        player: playerPda,
        inventory: inventoryPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([payer.payer])
      .rpc();

    console.log("Player initialized");

    const playerAccount = await program.account.player.fetch(playerPda);
    const inventoryAccount = await program.account.inventory.fetch(inventoryPda);

    assert.equal(playerAccount.deathCount, 0);
    assert.equal(playerAccount.summitCount, 0);
    assert.equal(inventoryAccount.equipments.length, 0);
  });

  it("Dead Increment", async () => {
    await program.methods
      .deadIncrement()
      .accountsPartial({
        player: playerPda,
        wallet: payer.publicKey,
      })
      .rpc();

    const playerAccount = await program.account.player.fetch(playerPda);

    assert.equal(playerAccount.deathCount, 1);
  })

  it("Reach Summit Increment", async () => {
    await program.methods
      .reachSummitIncrement()
      .accountsPartial({
        player: playerPda,
        wallet: payer.publicKey,
      })
      .rpc();

    const playerAccount = await program.account.player.fetch(playerPda);

    assert.equal(playerAccount.summitCount, 1);
  });

  it("Mint NFT to Player", async () => {
    const mint = anchor.web3.Keypair.generate();

    const ata = anchor.utils.token.associatedAddress({
      mint: mint.publicKey,
      owner: payer.publicKey,
    });

    const [metadataAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(METADATA_SEED),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    const [masterEditionAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.publicKey.toBuffer(),
        Buffer.from("edition"),
      ],
      TOKEN_METADATA_PROGRAM_ID
    )

    const context = {
      payer: payer.publicKey,
      mint: mint.publicKey,
      player: playerPda,
      ata,
      metadata: metadataAccount,
      masterEdition: masterEditionAccount,
      metadataProgram: TOKEN_METADATA_PROGRAM_ID,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      rent: SYSVAR_RENT_PUBKEY,
      playerInventory: inventoryPda,
    }

    const metadata = {
      name: "Crab#2",
      symbol: "DWC",
      uri: "https://raw.githubusercontent.com/Rayato159/my-metaplex-nft-collections/refs/heads/main/crab%232.json",
    };

    const tx = await program.methods
      .mintNftToPlayer(metadata.name, metadata.symbol, metadata.uri)
      .accounts(context)
      .preInstructions([
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
          units: 400_000,
        }),
      ])
      .signers([mint, payer.payer])
      .rpc({ commitment: "confirmed" });

    console.log(
      `mint nft tx: https://explorer.solana.com/tx/${tx}?cluster=devnet`
    );
    console.log(
      `minted nft: https://explorer.solana.com/address/${mint.publicKey}?cluster=devnet`
    );

    const inventoryAccount = await program.account.inventory.fetch(inventoryPda);

    assert.equal(inventoryAccount.equipments.length, 1);
    assert.equal(inventoryAccount.equipments[0].name, metadata.name);
    assert.ok(inventoryAccount.equipments[0].mint.equals(mint.publicKey));
  });

  // const airdrop = async (pubkey: anchor.web3.PublicKey) => {
  //   const sig = await anchor.getProvider().connection.requestAirdrop(pubkey, 0.01 * anchor.web3.LAMPORTS_PER_SOL);
  //   const blockhash = await anchor.getProvider().connection.getLatestBlockhash();
  //   await anchor.getProvider().connection.confirmTransaction({
  //     blockhash: blockhash.blockhash,
  //     lastValidBlockHeight: blockhash.lastValidBlockHeight,
  //     signature: sig,
  //   });
  // };
});
