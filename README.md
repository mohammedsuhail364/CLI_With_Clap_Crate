# CLI_With_Clap_Crate

A small Rust command-line application built while learning the [`clap`](https://crates.io/crates/clap) crate. It models registering people and their pets with a doctor's office.

At the moment it only parses arguments; it does not store or print anything yet.

## Requirements

- Rust toolchain with edition 2024 support (install via [rustup](https://rustup.rs))

## Build and run

```sh
cargo build
cargo run -- --help
```

## Usage

```
CLI_With_Clap_Crate [OPTIONS] [COMMAND]
```

### Subcommands

| Command         | Option                                                    | Description                      |
|-----------------|-----------------------------------------------------------|----------------------------------|
| `register-name` | `-f`, `--first-name` (aliases: `fname`, `firstname`)      | The person's first name (required) |
|                 | `-l`, `--last-name` (aliases: `lname`, `lastname`)        | The person's last name (required)  |
| `register-pet`  | `-p`, `--pet-name` (aliases: `pname`, `petname`)          | The pet's name (required)          |

### Global option

- `--fluffy` — an optional argument on the top-level command.

### Examples

```sh
cargo run -- register-name --first-name Jane --last-name Doe
cargo run -- register-name -f Jane -l Doe
cargo run -- register-pet --pet-name Rex
```

## Dependencies

- `clap` 4.x (with the `cargo` feature, used by the `command!()` macro to pull name, version and author from `Cargo.toml`)
