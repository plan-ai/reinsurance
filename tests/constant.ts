import { BN } from "@coral-xyz/anchor";
import { PublicKey } from "@saberhq/solana-contrib";

const insurerDescription: string =
  "https://en.aap.eu/chimpanzee-facts-and-figures/";
const insuranceId: string = "1";
const coverage: BN = new BN(1);
const premium: BN = new BN(1);
const minimumCommission: number = 1;
const deductible: BN = new BN(1);
const insuranceMetadataLink: string =
  "https://en.aap.eu/chimpanzee-facts-and-figures/";
const expiry: BN = new BN(1710076797);
const proposedCommision: BN = new BN(2);
const proposeduUndercollaterization: BN = new BN(10);
const proposalMetadataLink: string =
  "https://en.aap.eu/chimpanzee-facts-and-figures/";
const mintAmount: number = 10000;
const securityAmount: BN = new BN(9000);
const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);
const premiumMultiplier: BN = new BN(1);

export {
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
};
