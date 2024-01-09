import { BN } from "@coral-xyz/anchor";

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

export {
  insurerDescription,
  insuranceId,
  coverage,
  minimumCommission,
  premium,
  deductible,
  insuranceMetadataLink,
  expiry,
};
