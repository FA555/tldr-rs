# tldr-rs

A rust command-line client for [tldr-pages](https://github.com/tldr-pages/tldr).

## Building

### Requirements

- Rust / Cargo installed.

### Compiling

Just clone this repository and run `cargo build --release`:

``` bash
git clone git@github.com:FA555/tldr-rs.git
cd tldr-rs
cargo build --release
```

and the binary will be in `target/release/tldr`.

## Usage

```
Usage: tldr-rs [OPTIONS] [ITEM]

Arguments:
  [ITEM]

Options:
  -v, --version              Print version and exit
  -u, --update               Update local database
  -c, --clear-cache          Clear local database
  -l, --list                 List all entries in the local database
  -p, --platform <PLATFORM>  Select platform [possible values: linux, osx, sunos, windows, common]
  -r, --render <PATH>        Render a local page for testing purposes
  -h, --help                 Print help

```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
