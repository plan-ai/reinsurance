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
} from "./constant";
import { rpcConfig } from "./test_config";

describe("insurance", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Insurance as anchor.Program<Insurance>;
  const { web3 } = anchor;
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

    global.insurance_creator = insuranceCreator;
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
      global.insurance_creator.publicKey.toBuffer(),
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
        insuranceCreator: global.insurance_creator.publicKey,
        insurer: global.insurer,
        insurance: insurance,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([global.insurance_creator])
      .rpc(rpcConfig);
  });
});
