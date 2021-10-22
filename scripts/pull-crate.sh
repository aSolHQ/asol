#!/usr/bin/env sh

cd $(dirname $0)/..

mkdir -p artifacts/programs/

solana program dump CRATwLpu6YZEeiVq9ajjxs61wPQ9f29s1UoQR9siJCRs \
    artifacts/programs/crate_token.so --url devnet                                                                                                                           

solana program dump 1NKyU3qShZC3oJgvCCftAHDi5TFxcJwfyUz2FeZsiwE \
    artifacts/programs/crate_redeem_in_kind.so --url devnet                                                                                                                           
