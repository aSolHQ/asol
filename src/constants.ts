import { Token } from "@saberhq/token-utils";
import { PublicKey } from "@solana/web3.js";

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

export const LAMPORTS_DECIMALS = 9;
