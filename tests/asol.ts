import { EventParser } from "@project-serum/anchor";
import { expectTX } from "@saberhq/chai-solana";
import { createInitMintInstructions } from "@saberhq/token-utils";
import type { PublicKey } from "@solana/web3.js";
import { Keypair } from "@solana/web3.js";
import { expect } from "chai";

import type { AddStakePoolEvent } from "../src";
import { AccountingMethods } from "../src";
import { makeSDK } from "./workspace";

describe("aSOL", () => {
  const sdk = makeSDK();
  let aggregate: PublicKey;

  beforeEach(async () => {
    const mintKP = Keypair.generate();
    const { tx: createTX, aggregateKey } = await sdk.newAggregate({
      mintKP,
    });
    aggregate = aggregateKey;
    await expectTX(createTX, "Create Crate Token").to.be.fulfilled;
  });

  it("add stake pools", async () => {
    const lidoKP = Keypair.generate();
    const fakeLido = await createInitMintInstructions({
      provider: sdk.provider,
      mintKP: lidoKP,
      decimals: 9,
    });
    await expectTX(fakeLido).to.be.fulfilled;

    const { tx } = await sdk.addStakePool({
      aggregate,
      mint: lidoKP.publicKey,
      method: AccountingMethods.Lido,
    });
    const result = await tx.send();
    await expectTX(result).to.be.fulfilled;

    const parser = new EventParser(sdk.program.programId, sdk.program.coder);
    const logs = (await result.wait()).response.meta?.logMessages ?? [];

    parser.parseLogs(logs, (ev) => {
      const event: AddStakePoolEvent = ev as unknown as AddStakePoolEvent;

      expect(event.name).to.eq("AddStakePoolEvent");
      expect(event.data.aggregate).to.eqAddress(aggregate);
      expect(event.data.curator).to.eqAddress(sdk.provider.wallet.publicKey);
      expect(event.data.mint).to.eqAddress(lidoKP.publicKey);
      expect(event.data.accountingMethod).to.deep.eq(AccountingMethods.Lido);
    });
  });

  // todo: figure out how to mock lido and marinade locally
});
