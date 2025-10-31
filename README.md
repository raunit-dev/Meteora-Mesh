# Meteora Mesh

A Solana Anchor monorepo for developing and managing one or more on-chain programs in Rust. This workspace uses a shared Cargo configuration with an optimized release profile and a TypeScript toolchain for client code and tests. Currently includes the `meteora-node` program.

## Features

- Anchor workspace layout (programs, tests, migrations)
- Rust Cargo workspace that automatically includes `programs/*`
- Tuned release profile:
  - `overflow-checks = true`
  - `lto = "fat"`
  - `codegen-units = 1`
- TypeScript toolchain (Yarn) for client tests
- Ready for localnet/devnet deploy flows

## Repository structure

```
.
├─ Anchor.toml          # Anchor configuration (clusters, program IDs, etc.)
├─ Cargo.toml           # Workspace + release profile
├─ Cargo.lock
├─ package.json         # TypeScript client/testing deps
├─ tsconfig.json
├─ yarn.lock
├─ programs/
│  └─ meteora-node/     # On-chain Anchor program (Rust)
├─ tests/               # Anchor tests (TypeScript)
└─ migrations/          # Anchor migration scripts
```

## Prerequisites

- Rust (via rustup)
- Solana CLI
- Anchor CLI
- Node.js (LTS) and Yarn

Quick setup:
```
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Solana
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Anchor (uses npm)
npm i -g @coral-xyz/anchor-cli

# Yarn (if not installed)
npm i -g yarn
```

Verify installs:
```
rustc --version
solana --version
anchor --version
node --version
yarn --version
```

## Install dependencies

```
yarn install
```

## Build

Anchor build (recommended):
```
anchor build
```

Cargo workspace build (debug):
```
cargo build
```

Cargo workspace build (release):
```
cargo build --release
```

## Localnet

Start a local validator in a separate terminal:
```
solana-test-validator
```

Point your CLI at localnet:
```
solana config set --url localhost
```

(Optional) Create a keypair if you don’t have one:
```
solana-keygen new
```

## Deploy

Ensure the program IDs in `Anchor.toml` are correct for the target cluster. Then:

Localnet:
```
anchor deploy
```

Devnet:
```
solana config set --url https://api.devnet.solana.com
anchor deploy
```

## Test

Anchor will build and run the tests under `tests/`:
```
anchor test
```

Run TypeScript tests directly (if you prefer invoking the test runner configured in package.json):
```
yarn test
```

## Lint and format

Rust:
```
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
```

TypeScript (format via Prettier):
```
yarn format
# or, if no script is defined:
npx prettier --write .
```

## Configuration notes

- Workspace members are auto-included from `programs/*` via `Cargo.toml`.
- Release profile in `Cargo.toml` is tuned for safety and performance:
  - `overflow-checks = true` keeps integer overflow checks enabled in release.
  - `lto = "fat"` and `codegen-units = 1` favor tighter, faster binaries over compile speed.
- `Anchor.toml` controls the cluster targets and program IDs; update as needed per environment.

## Common commands

```
# Build all programs
anchor build

# Run tests (builds, spins up local validator if needed, runs TS tests)
anchor test

# Deploy to current Solana cluster (check `solana config get`)
anchor deploy

# Switch cluster
solana config set --url localhost
solana config set --url https://api.devnet.solana.com

# Key management
solana-keygen new
solana address
```

## License

TBD. Common choices: MIT, Apache-2.0, or MIT OR Apache-2.0.
