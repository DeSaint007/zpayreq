# zpayreq — Zcash Payment-Request Builder

Standalone implementation for the Gleyo Payment-Request Builder quest.
Participant: **the_grand_puppeteer**. GitHub: **DeSaint007**. AI-assisted.

## Build and run
Requires Rust 1.85.1+ (tested with 1.95.0).
```sh
cargo build --locked
./target/debug/zpayreq '<address>' '1.25'
./target/debug/zpayreq '<address>' '1.25' --shielded
cargo test --locked --all-targets
```
Success prints a single-recipient ZIP-321 URI:
`zcash:<address>?amount=1.25`. No funds are sent.

Address encodings and checksums are validated by the official zcash_address
crate. Transparent P2PKH/P2SH, Sapling and Unified addresses are supported.
Sprout and TEX are outside this quest's supported set. --shielded rejects
transparent-only recipients. Validation does not prove ownership or that
shielded receiver key bytes correspond to a spendable destination.

Amounts use integer zatoshis, not floating point. Accepted amounts are
positive fixed-point decimals with at most eight fractional digits and no
more than 21 million ZEC. Exponent notation, NaN and infinity are rejected.
Trailing fractional zeroes are normalized in the generated URI.

## Error classes
Errors are JSON on stderr with a nonzero exit status; successful URI output
is on stdout. Validation order: address, shielded policy, amount.

| Code | Meaning |
| --- | --- |
| ERR_USAGE | Wrong number of arguments |
| ERR_INVALID_ADDRESS | Unsupported or malformed address/checksum |
| ERR_SHIELDED_REQUIRED | Transparent recipient with --shielded |
| ERR_AMOUNT_FORMAT | Not an unsigned fixed-point decimal |
| ERR_AMOUNT_NON_POSITIVE | Zero or negative amount |
| ERR_AMOUNT_PRECISION | More than eight decimal places |
| ERR_AMOUNT_RANGE | Overflow or above 21 million ZEC |

## Tests
Unit and CLI tests cover valid Sapling and Unified requests, transparent
policy rejection, malformed addresses and every amount error class.
CI runs rustfmt, Clippy and cargo test. These are public project tests;
no unseen quest grader or hidden suite is claimed to have passed.

References: https://docs.rs/zcash_address/0.13.0/zcash_address/
and https://zips.z.cash/zip-0321
