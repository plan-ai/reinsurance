import * as anchor from "@project-serum/anchor";
import { Insurance } from "../target/types/insurance";

// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env());

//testing defios workspace here
const program = anchor.workspace.Insurance as anchor.Program<Insurance>;
const {
  provider: { connection },
} = program;
const { web3 } = anchor;

async function create_keypair() {
  const keypair = web3.Keypair.generate();
  await connection.confirmTransaction(
    {
      signature: await connection.requestAirdrop(
        keypair.publicKey,
        web3.LAMPORTS_PER_SOL
      ),
      ...(await connection.getLatestBlockhash()),
    },
    "confirmed"
  );
  return keypair;
}

async function get_pda_from_seeds(seeds) {
  return web3.PublicKey.findProgramAddressSync(seeds, program.programId)[0];
}

async function get_metadata_account(mintKeypair) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      constant.TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mintKeypair.toBuffer(),
    ],
    constant.TOKEN_METADATA_PROGRAM_ID
  )[0];
}

export { create_keypair, get_pda_from_seeds };
