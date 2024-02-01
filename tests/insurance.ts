import * as anchor from "@coral-xyz/anchor";
import { Insurance } from "../target/types/insurance";
import { create_keypair, get_pda_from_seeds } from "./helper";
import {
  insurerDescription,
  insuranceId,
  coverage,
  minimumCommission,
  premium,
  deductible,
  insuranceMetadataLink,
  expiry,
  proposedCommision,
  proposeduUndercollaterization,
  proposalMetadataLink,
  mintAmount,
  securityAmount,
  TOKEN_METADATA_PROGRAM_ID,
  premiumMultiplier,
} from "./constant";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  createMint,
  mintTo,
  transfer,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { rpcConfig } from "./test_config";
import { BN } from "bn.js";

describe("insurance", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Insurance as anchor.Program<Insurance>;
  const { web3 } = anchor;
  const {
    provider: { connection },
  } = program;
  let global: any = {};

  it("Create insurer!", async () => {
    const insuranceCreator = await create_keypair();
    const insurer = await get_pda_from_seeds([
      insuranceCreator.publicKey.toBuffer(),
    ]);
    await program.methods
      .registerInsurer(insurerDescription)
      .accounts({
        insuranceCreator: insuranceCreator.publicKey,
        insurer: insurer,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([insuranceCreator])
      .rpc(rpcConfig);

    global.insuranceCreator = insuranceCreator;
    global.insurer = insurer;
  });

  it("Creates a LP!", async () => {
    const lpCreator = await create_keypair();
    const lp = await get_pda_from_seeds([lpCreator.publicKey.toBuffer()]);
    await program.methods
      .registerLp()
      .accounts({
        lpCreator: lpCreator.publicKey,
        lp: lp,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([lpCreator])
      .rpc(rpcConfig);

    global.lpCreator = lpCreator;
    global.lp = lp;
  });

  it("Registers an insurance", async () => {
    const insurance = await get_pda_from_seeds([
      global.insuranceCreator.publicKey.toBuffer(),
      Buffer.from(insuranceId),
    ]);

    await program.methods
      .registerInsurance(
        insuranceId,
        coverage,
        premium,
        minimumCommission,
        deductible,
        expiry,
        insuranceMetadataLink
      )
      .accounts({
        insuranceCreator: global.insuranceCreator.publicKey,
        insurer: global.insurer,
        insurance: insurance,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([global.insuranceCreator])
      .rpc(rpcConfig);
    global.insurance = insurance;
  });
  it("Sends an insurance proposal", async () => {
    const proposal = await get_pda_from_seeds([
      global.lpCreator.publicKey.toBuffer(),
      global.insurance.toBuffer(),
    ]);

    await program.methods
      .sendInsuranceProposal(
        proposedCommision,
        proposeduUndercollaterization,
        proposalMetadataLink
      )
      .accounts({
        lpCreator: global.lpCreator.publicKey,
        lp: global.lp,
        insurance: global.insurance,
        proposal: proposal,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([global.lpCreator])
      .rpc(rpcConfig);
    global.proposal = proposal;
  });
  it("Creates and adds security using artificial token", async () => {
    // note: This will not work on pushed contracts
    const mintAddress = await createMint(
      connection,
      global.lpCreator,
      global.lpCreator.publicKey,
      global.lpCreator.publicKey,
      6
    );

    const lpCreatorTokenAddress = await getOrCreateAssociatedTokenAccount(
      connection,
      global.lpCreator,
      mintAddress,
      global.lpCreator.publicKey
    );

    await mintTo(
      connection,
      global.lpCreator,
      mintAddress,
      lpCreatorTokenAddress.address,
      global.lpCreator,
      mintAmount
    );

    const lpMintAccount = await getAssociatedTokenAddress(
      mintAddress,
      global.lp,
      true
    );

    await program.methods
      .addSecurity(securityAmount)
      .accounts({
        lpCreator: global.lpCreator.publicKey,
        lp: global.lp,
        lpCreatorUsdcAccount: lpCreatorTokenAddress.address,
        lpUsdcAccount: lpMintAccount,
        usdcMint: mintAddress,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([global.lpCreator])
      .rpc(rpcConfig);
    global.mintAddress = mintAddress;
    global.lpCreatorTokenAddress = lpCreatorTokenAddress;
  });
  it("Accept reinsurance proposal", async () => {
    await program.methods
      .acceptReinsuranceProposal()
      .accounts({
        insuranceCreator: global.insuranceCreator.publicKey,
        insurance: global.insurance,
        lp: global.lp,
        proposal: global.proposal,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([global.insuranceCreator])
      .rpc(rpcConfig);
  });
  it("Tokenise LP", async () => {
    const tokenisedMint = await get_pda_from_seeds([
      Buffer.from("i_am_in_love"),
      Buffer.from("withacriminl"),
      global.lp.toBuffer(),
    ]);

    const lpCreatorTokenisedAccount = await getAssociatedTokenAddress(
      tokenisedMint,
      global.lpCreator.publicKey
    );

    await program.methods
      .tokeniseLp(null, null, null)
      .accounts({
        lpCreator: global.lpCreator.publicKey,
        lp: global.lp,
        tokenisedMint: tokenisedMint,
        lpCreatorTokenisedAccount: lpCreatorTokenisedAccount,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([global.lpCreator])
      .rpc(rpcConfig);
  });
  it("Pay premium", async () => {
    const premiumVault = await get_pda_from_seeds([
      Buffer.from("premium"),
      global.insurance.toBuffer(),
      global.proposal.toBuffer(),
    ]);

    const premiumVaultTokenAccount = await getAssociatedTokenAddress(
      global.mintAddress,
      premiumVault,
      true
    );

    const insuranceCreatorTokenAccount =
      await getOrCreateAssociatedTokenAccount(
        connection,
        global.insuranceCreator,
        global.mintAddress,
        global.insuranceCreator.publicKey
      );

    await transfer(
      connection,
      global.lpCreator,
      global.lpCreatorTokenAddress.address,
      insuranceCreatorTokenAccount.address,
      global.lpCreator.publicKey,
      1
    );

    await program.methods
      .payPremium(premiumMultiplier)
      .accounts({
        insuranceCreator: global.insuranceCreator.publicKey,
        insuranceCreatorTokenAccount: insuranceCreatorTokenAccount.address,
        insurance: global.insurance,
        premiumVault: premiumVault,
        premiumVaultTokenAccount: premiumVaultTokenAccount,
        proposal: global.proposal,
        usdcMint: global.mintAddress,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([global.insuranceCreator])
      .rpc(rpcConfig);
  });
});
