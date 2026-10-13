# Contributing

## Development

You need a stable Rust toolchain, `make`, a C compiler and `libclang` (for
bindgen). The machine crates also need the Cartesi Machine emulator v0.21
installed (see [`cartesi-machine-sys`](crates/cartesi-machine-sys/README.md)).
Their tests boot images you prepare once with the `cartesi-machine` CLI, by
running `scripts/prepare-test-images.sh`.

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo doc --workspace --no-deps --locked
```

On the host, `cartesi-libcmt-sys` links libcmt's mock I/O backend. To build
the guest crates for the Cartesi Machine, use [cross] with the image pinned in
`Cross.toml`:

```sh
cross build --target riscv64gc-unknown-linux-musl -p cartesi-rollup
```

Keep the code buildable with the minimum supported Rust version
(`rust-version` in `Cargo.toml`). Raising it is a minor change.

## Changesets

Versions and changelogs are managed by [Sampo] from changesets: markdown files
in `.sampo/changesets/` that say which crates a change affects, how big the
version bump is, and what users should read in the changelog. Commit messages
are not used for releases, so write them for contributors.

Add a changeset to any pull request that changes a published crate in a way
users can notice:

```sh
sampo add
```

or write the file by hand, for example `.sampo/changesets/gio-timeout.md`:

```md
---
cargo/cartesi-rollup: minor
---

Add a timeout to `Rollup::gio`.
```

- Pick the bump by impact: `patch` for fixes, `minor` for new features,
  `major` for breaking changes. While a crate is `0.x`, use `minor` for
  breaking changes and `patch` for everything else.
- Write the description for users of the crate, in the imperative mood. It
  becomes the changelog entry as written.
- One changeset can name several crates. A crate that depends on a bumped
  crate in this workspace is patched automatically.
- Internal changes (CI, tests, refactors with no visible effect) need no
  changeset.

Install Sampo with `cargo install sampo --locked`.

## Releasing

Releases are automated by `.github/workflows/release.yml`:

1. While changesets are pending on `main`, the workflow keeps a
   **Release** pull request open. It bumps the versions, updates each crate's
   `CHANGELOG.md` and `Cargo.lock`, and consumes the changesets.
2. Merging that pull request publishes the bumped crates to crates.io, tags
   each release as `<crate>-v<version>` and creates the GitHub releases.

Publishing uses crates.io [trusted publishing], so the repository holds no
crates.io token. The **first** version of a new crate cannot be published that
way: a maintainer publishes it once by hand (for example with `sampo publish`
and a personal token in `CARGO_REGISTRY_TOKEN`). Then they configure the crate's
trusted publisher on crates.io, under *Settings → Trusted Publishing*:

- Repository owner: `cartesi`
- Repository name: `rollups-rs`
- Workflow filename: `release.yml`
- Environment: `release`

Until a new crate's first version is published this way, the release workflow
fails on `main`.

[cross]: https://github.com/cross-rs/cross
[Sampo]: https://github.com/bruits/sampo
[trusted publishing]: https://crates.io/docs/trusted-publishing
