import {
  CRATE_ADDRESSES,
  CRATE_REDEEM_IN_KIND_WITHDRAW_AUTHORITY,
  CrateSDK,
  generateCrateAddress,
} from "@crateprotocol/crate-sdk";
import { Program, Provider as AnchorProvider } from "@project-serum/anchor";
import type { Provider } from "@saberhq/solana-contrib";
import {
  SignerWallet,
  SolanaProvider,
  TransactionEnvelope,
} from "@saberhq/solana-contrib";
import type { TokenAmount } from "@saberhq/token-utils";
import {
  createInitMintInstructions,
  getATAAddresses,
  getOrCreateATAs,
  TOKEN_PROGRAM_ID,
} from "@saberhq/token-utils";
import type { PublicKey, Signer } from "@solana/web3.js";
import { Keypair, SystemProgram } from "@solana/web3.js";

import { generateStakePoolAddress } from ".";
import {
  ASOL_PROGRAM_ID,
  LAMPORTS_DECIMALS,
  LIDO_STAKED_SOL,
  MARINADE_STAKED_SOL,
  MARINADE_STATE_ACCOUNT,
  SOLIDO_ACCOUNT,
  STAKE_POOL_TOKENS,
} from "./constants";
import { generateAggregateAddress } from "./pda";
import type {
  AccountingMethod,
  AggregateData,
  ASolProgram,
} from "./programs/asol";
import { AsolJSON } from "./programs/asol";

/**
 * Javascript SDK for interacting with Crate tokens.
 */
export class ASolSDK {
  /**
   * Reference to the Crate SDK.
   */
  public readonly crate: CrateSDK;

  constructor(
    public readonly provider: Provider,
    public readonly program: ASolProgram
  ) {
    this.crate = CrateSDK.init(provider);
  }

  /**
   * Initialize from a Provider
   * @param provider
   * @param asolProgramID
   * @returns
   */
  static init(
    provider: Provider,
    asolProgramID: PublicKey = ASOL_PROGRAM_ID
  ): ASolSDK {
    return new ASolSDK(
      provider,
      new Program(
        AsolJSON,
        asolProgramID,
        new AnchorProvider(provider.connection, provider.wallet, provider.opts)
      ) as unknown as ASolProgram
    );
  }

  /**
   * Creates a new instance of the SDK with the given keypair.
   */
  public withSigner(signer: Signer): ASolSDK {
    return ASolSDK.init(
      new SolanaProvider(
        this.provider.connection,
        this.provider.broadcaster,
        new SignerWallet(signer),
        this.provider.opts
      )
    );
  }

  /**
   * Creates a new Aggregate.
   * @returns
   */
  async newAggregate({
    mintKP = Keypair.generate(),
    admin = this.provider.wallet.publicKey,
    payer = this.provider.wallet.publicKey,
  }: {
    mintKP?: Keypair;
    admin?: PublicKey;
    payer?: PublicKey;
  } = {}): Promise<{
    tx: TransactionEnvelope;
    aggregateKey: PublicKey;
    crateKey: PublicKey;
  }> {
    const [crateKey, crateBump] = await generateCrateAddress(mintKP.publicKey);
    const [aggregateKey, aggBump] = await generateAggregateAddress(crateKey);
    const initMintTX = await createInitMintInstructions({
      provider: this.provider,
      mintKP,
      decimals: LAMPORTS_DECIMALS, // lamports
      mintAuthority: crateKey,
      freezeAuthority: crateKey,
    });
    const newAggregateTX = new TransactionEnvelope(this.provider, [
      this.program.instruction.newAggregate(aggBump, crateBump, {
        accounts: {
          crateMint: mintKP.publicKey,
          payer,
          redeemInKind: CRATE_REDEEM_IN_KIND_WITHDRAW_AUTHORITY,
          aggregate: aggregateKey,
          crateToken: crateKey,
          admin,
          systemProgram: SystemProgram.programId,
          crateTokenProgram: CRATE_ADDRESSES.CrateToken,
        },
      }),
    ]);
    return { tx: initMintTX.combine(newAggregateTX), aggregateKey, crateKey };
  }

  /**
   * Adds a new stake pool.
   * @returns
   */
  async addStakePool({
    aggregate,
    mint,
    method,
    curator = this.provider.wallet.publicKey,
    payer = this.provider.wallet.publicKey,
  }: {
    aggregate: PublicKey;
    mint: PublicKey;
    method: AccountingMethod;
    curator?: PublicKey;
    payer?: PublicKey;
  }): Promise<{ tx: TransactionEnvelope; stakePoolKey: PublicKey }> {
    const [stakePool, bump] = await generateStakePoolAddress(
      aggregate,
      mint,
      this.program.programId
    );
    const newStakePoolTX = new TransactionEnvelope(this.provider, [
      this.program.instruction.addStakePool(bump, method, {
        accounts: {
          aggregate,
          stakePool,
          mint,
          curator,
          payer,
          systemProgram: SystemProgram.programId,
        },
      }),
    ]);
    return { tx: newStakePoolTX, stakePoolKey: stakePool };
  }

  /**
   * Mints tokens.
   * @returns
   */
  async mintASol({
    aggregateKey,
    amount,
    depositor = this.provider.wallet.publicKey,
  }: {
    aggregateKey: PublicKey;
    amount: TokenAmount;
    depositor?: PublicKey;
  }): Promise<TransactionEnvelope> {
    const depositMint = amount.token.mintAccount;
    const method = depositMint.equals(MARINADE_STAKED_SOL)
      ? "mintMarinade"
      : depositMint.equals(LIDO_STAKED_SOL)
      ? "mintLido"
      : null;
    if (!method) {
      throw new Error("Invalid mint.");
    }

    const aggregate = (await this.program.account.aggregate.fetchNullable(
      aggregateKey
    )) as AggregateData;
    if (!aggregate) {
      throw new Error("No aggregate found.");
    }

    const crate = await this.crate.fetchCrateTokenData(aggregate.crateToken);
    if (!crate) {
      throw new Error("No crate found.");
    }

    const stakePoolATAs = await getATAAddresses({
      mints: {
        marinade: MARINADE_STAKED_SOL,
        lido: LIDO_STAKED_SOL,
        input: amount.token.mintAccount,
      },
      owner: aggregate.crateToken,
    });

    const depositorATAs = await getOrCreateATAs({
      provider: this.provider,
      mints: {
        input: amount.token.mintAccount,
        crate: crate.mint,
      },
      owner: depositor,
    });

    const [stakePool] = await generateStakePoolAddress(
      aggregateKey,
      amount.token.mintAccount
    );

    const mintTX = new TransactionEnvelope(this.provider, [
      ...(depositorATAs.createAccountInstructions.crate
        ? [depositorATAs.createAccountInstructions.crate]
        : []),
      this.program.instruction[method](amount.toU64(), {
        accounts: {
          sync: {
            marinade: {
              marinade: MARINADE_STATE_ACCOUNT,
              marinadeStakePoolTokens: stakePoolATAs.accounts.marinade.address,
            },
            lido: {
              lido: SOLIDO_ACCOUNT,
              lidoStakePoolTokens: stakePoolATAs.accounts.lido.address,
            },
          },
          mintAsol: {
            aggregate: aggregateKey,
            stakePool,
            crateToken: aggregate.crateToken,
            crateMint: crate.mint,
            tokenProgram: TOKEN_PROGRAM_ID,
            crateTokenProgram: CRATE_ADDRESSES.CrateToken,

            depositor,
            depositorSource: depositorATAs.accounts.input,
            stakePoolTokens: stakePoolATAs.accounts.input.address,
            mintDestination: depositorATAs.accounts.crate,
          },
        },
      }),
    ]);

    return mintTX;
  }

  /**
   * Redeems Crate tokens for the underlying tokens.
   */
  async redeem({
    amount,
    owner = this.provider.wallet.publicKey,
  }: {
    amount: TokenAmount;
    owner?: PublicKey;
  }): Promise<TransactionEnvelope> {
    return await this.crate.redeem({
      amount,
      owner,
      underlyingTokens: Object.values(STAKE_POOL_TOKENS),
    });
  }
}
