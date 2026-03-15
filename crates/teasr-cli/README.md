# teasr-cli

The `teasr` binary. A thin CLI wrapper around `teasr-core` using clap derive.

## Installation

**Shell installer:**

```bash
curl -fsSL https://raw.githubusercontent.com/urmzd/tease/main/install.sh | bash
```

**Cargo:**

```bash
cargo install teasr-cli
```

**GitHub Action:** see the [root README](../../README.md#ci-integration).

## Usage

```
teasr [OPTIONS]

Options:
  -c, --config <PATH>      Path to teasr.toml (default: auto-discover from cwd)
  -o, --output <DIR>       Output directory (overrides config)
      --formats <FMT,...>  Output formats: png, gif (overrides config)
      --verbose            Enable debug logging
      --timeout <MS>       Global timeout in milliseconds [default: 60000]
  -h, --help               Print help
  -V, --version            Print version
```

## Config Discovery

When `--config` is not provided, teasr walks up from the current directory to the filesystem root looking for `teasr.toml`. This means you can run `teasr` from any subdirectory of your project.

## Overrides

`--output` and `--formats` override the corresponding values from `teasr.toml`. Formats are comma-separated: `--formats png,gif`.

## Timeout

`--timeout` wraps the entire run. If teasr does not complete within the specified milliseconds the process exits with an error. The default is 60 seconds. Set it higher for slow CI environments or large scene lists.

## Logging

Log level defaults to `info`. Use `--verbose` for `debug` output, or set `RUST_LOG` directly for fine-grained control (e.g. `RUST_LOG=teasr_core=debug`).

## License

Apache-2.0
