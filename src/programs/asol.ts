import type { AnchorTypes } from "@saberhq/anchor-contrib";

import type { AsolIDL } from "../idls/asol";

export * from "../idls/asol";

type ASolTypes = AnchorTypes<
  AsolIDL,
  {
    aggregate: AggregateData;
    stakePool: StakePoolData;
  },
  {
    AccountingMethod: AccountingMethod;
    StakePoolMeta: StakePoolMeta;
    StakePoolSnapshot: StakePoolSnapshot;
    StakePoolStateSnapshot: StakePoolStateSnapshot;
    StakePoolStats: StakePoolStats;
    Snapshot: Snapshot;
    SOL: SOLValue;
    ASOL: ASOLValue;
  }
>;

export type AggregateData = ASolTypes["Accounts"]["Aggregate"];
export type StakePoolData = ASolTypes["Accounts"]["StakePool"];

export type AccountingMethod =
  typeof AccountingMethods[keyof typeof AccountingMethods];

export const AccountingMethods = {
  Marinade: {
    marinade: {},
  },
  Lido: {
    lido: {},
  },
} as const;

export type SOLValue = ASolTypes["Defined"]["SOL"];
export type ASOLValue = ASolTypes["Defined"]["ASOL"];

export type StakePoolMeta = ASolTypes["Defined"]["StakePoolMeta"];
export type StakePoolSnapshot = ASolTypes["Defined"]["StakePoolSnapshot"];
export type StakePoolStateSnapshot =
  ASolTypes["Defined"]["StakePoolStateSnapshot"];
export type StakePoolStats = ASolTypes["Defined"]["StakePoolStats"];
export type Snapshot = ASolTypes["Defined"]["Snapshot"];

export type ASolProgram = ASolTypes["Program"];

export type NewAggregateEvent = ASolTypes["Events"]["NewAggregateEvent"];
export type AddStakePoolEvent = ASolTypes["Events"]["AddStakePoolEvent"];
export type SetCuratorEvent = ASolTypes["Events"]["SetCuratorEvent"];
export type MintASolEvent = ASolTypes["Events"]["MintASolEvent"];
export type AggregateInfoEvent = ASolTypes["Events"]["AggregateInfoEvent"];

export type SyncAndMintAccounts =
  ASolTypes["Instructions"]["mintLido"]["accounts"];
