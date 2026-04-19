# teasr

The `teasr` binary. A thin CLI wrapper around `teasr-core` using clap derive.

## Installation

**Shell installer:**

```bash
curl -fsSL https://raw.githubusercontent.com/urmzd/teasr/main/install.sh | bash
```

**Cargo:**

```bash
cargo install teasr
```

**GitHub Action:** see the [root README](../../README.md#ci-integration).

## Usage

```
teasr [COMMAND]

Commands:
  run     Run capture scenes from teasr.toml (alias: `showme`)
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### `teasr run`

```
teasr run [OPTIONS]

Options:
  -c, --config <PATH>      Path to teasr.toml (default: auto-discover from cwd)
  -o, --output <DIR>       Output directory (overrides config)
      --formats <FMT,...>  Output formats: png, gif, mp4 (overrides config)
      --verbose            Enable debug logging
      --timeout <MS>       Global timeout in milliseconds [default: 60000]
      --fps <N>            Frames per second (overrides config)
      --seconds <N>        Target output duration in seconds (overrides config)
      --scene-timeout <N>  Per-scene wall-clock timeout in seconds (overrides config)
  -h, --help               Print help
```

## Config Discovery

When `--config` is not provided, teasr walks up from the current directory to the filesystem root looking for `teasr.toml`. This means you can run `teasr` from any subdirectory of your project.

## Overrides

`--output` and `--formats` override the corresponding values from `teasr.toml`. Formats are comma-separated: `--formats png,gif,mp4`.

## Timeout

`--timeout` wraps the entire run. If teasr does not complete within the specified milliseconds the process exits with an error. The default is 60 seconds. Set it higher for slow CI environments or large scene lists.

## Logging

Log level defaults to `info`. Use `--verbose` for `debug` output, or set `RUST_LOG` directly for fine-grained control (e.g. `RUST_LOG=teasr_core=debug`).

## License

Apache-2.0
