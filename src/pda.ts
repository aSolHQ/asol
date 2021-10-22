import { utils } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";

import { ASOL_PROGRAM_ID } from "./constants";

export const generateAggregateAddress = (
  crateToken: PublicKey,
  programID: PublicKey = ASOL_PROGRAM_ID
): Promise<[PublicKey, number]> => {
  return PublicKey.findProgramAddress(
    [utils.bytes.utf8.encode("Aggregate"), crateToken.toBuffer()],
    programID
  );
};

export const generateStakePoolAddress = (
  aggregate: PublicKey,
  mint: PublicKey,
  programID: PublicKey = ASOL_PROGRAM_ID
): Promise<[PublicKey, number]> => {
  return PublicKey.findProgramAddress(
    [
      utils.bytes.utf8.encode("StakePool"),
      aggregate.toBuffer(),
      mint.toBuffer(),
    ],
    programID
  );
};
