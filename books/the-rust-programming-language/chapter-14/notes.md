# Notes

## Customizing Builds with Release Profiles

General notes:
- Release profiles can be configured in `Cargo.toml`.
- The `opt-level` configuration controls the number of optimizations.

## Publishing a Crate to Crates.io

General ideas:
- It's possible to create documentation comment with `///`.
- The documentation support markdown.
- Running `cargo doc --open` builds and opens the crate's documentation.
- It's common to create the sections `Panics`, `Errors`, `Safety`, and
  `Examples`.
- Running `cargo test` executes the code examples in the documentation.

## Cargo Workspaces

General ideas:
- A workspace is a set of packages that share the same lockfile and output
  directory.
- Because there's only one lockfile, all crates use the same dependencies'
  versions.

## Installing Binaries with cargo install

General ideas:
- We can install binary crates with `cargo install`.
- If not customized, binaries are added to `$HOME/.cargo/bin`.

## Extending Cargo with Custom Commands

If a binary is called `cargo-xyz`, we can rust `cargo xyz`.