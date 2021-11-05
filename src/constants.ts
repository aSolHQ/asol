import { Token } from "@saberhq/token-utils";
import { PublicKey } from "@solana/web3.js";

/**
 * Program ID of the aSOL program.
 */
export const ASOL_PROGRAM_ID = new PublicKey(
  "AURUqAcTZP8mhR6sWVxWyfBbpJRj4A3qqeFzLNhrwayE"
);

export const MARINADE_STATE_ACCOUNT = new PublicKey(
  "8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC"
);

export const SOLIDO_ACCOUNT = new PublicKey(
  "49Yi1TKkNyYjPAFdR9LBvoHcUjuPX4Df5T5yv39w2XTn"
);

export const MARINADE_STAKED_SOL = new PublicKey(
  "mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So"
);

export const LIDO_STAKED_SOL = new PublicKey(
  "7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj"
);

/**
 * Supported stake pools.
 */
export const STAKE_POOL_TOKENS = {
  LIDO: new Token({
    chainId: 101,
    address: LIDO_STAKED_SOL.toString(),
    symbol: "stSOL",
    name: "Lido Staked SOL",
    decimals: 9,
    logoURI:
      "https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj/logo.png",
    tags: [],
    extensions: {
      website: "https://solana.lido.fi/",
      twitter: "https://twitter.com/LidoFinance/",
    },
  }),
  MARINADE: new Token({
    chainId: 101,
    address: MARINADE_STAKED_SOL.toString(),
    symbol: "mSOL",
    name: "Marinade staked SOL (mSOL)",
    decimals: 9,
    logoURI:
      "https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So/logo.png",
    tags: [],
    extensions: {
      coingeckoId: "msol",
      website: "https://marinade.finance",
      twitter: "https://twitter.com/MarinadeFinance",
      discord: "https://discord.gg/mGqZA5pjRN",
      medium: "https://medium.com/marinade-finance",
      github: "https://github.com/marinade-finance",
    },
  }),
};

/**
 * Number of decimals in one SOL.
 */
export const LAMPORTS_DECIMALS = 9;

/**
 * Mint address of the aSOL token.
 */
export const ASOL_MINT = new PublicKey(
  "ASoLXbfe7cd6igh5yiEsU8M7FW64QRxPKkxk7sjAfond"
);

/**
 * The aSOL token.
 */
export const ASOL_TOKEN = new Token({
  chainId: 101,
  address: ASOL_MINT.toString(),
  symbol: "aSOL",
  name: "aSOL Aggregate Solana Stake Pool",
  decimals: LAMPORTS_DECIMALS,
  logoURI: "https://asol.so/images/asol-token-icon.svg",
  tags: [],
  extensions: {
    coingeckoId: "solana",
    description: "aSOL is the standard for transacting with staked SOL tokens.",
    website: "https://asol.so",
    twitter: "https://twitter.com/aSOLprotocol",
    github: "https://github.com/aSolHQ",
  },
});
