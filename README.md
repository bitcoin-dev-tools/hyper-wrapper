# hyper-wrapper

A Rust-based wrapper for the [hyperfine](https://github.com/sharkdp/hyperfine) benchmarking tool that allows you to specify benchmark configurations using JSON files.

## Prerequisites

- Rust 1.70.0 or later
- [hyperfine](https://github.com/sharkdp/hyperfine) installed and available in your `PATH`

## Installation

```bash
cargo install --path .
```

## Usage

1. Create a JSON configuration file (e.g., `bench-config.json`):

```json
{
  "command": "bitcoind -datadir=/mnt/bench/.bitcoin -printtoconsole=0 -dbcache={dbcache} -stopatheight={height}",
  "parameter-list": [
    {
      "name": "height",
      "values": "1000,20000,300000"
    },
    {
      "name": "dbcache",
      "values": "100,450"
    }
  ],
  "prepare": "sync && rm -Rf /mnt/bench/.bitcoin/*",
  "cleanup": "",
  "runs": 1,
  "show-output": true,
  "export-json": "results.json"
}
```

2. Run the wrapper:

```bash
hyper-wrapper bench-config.json
```

## Configuration Options

The JSON configuration file supports the following options:

| Field | Type | Description |
|-------|------|-------------|
| `command` | string | The command to benchmark with parameter placeholders |
| `parameter-list` | array? | List of parameters and their values for benchmarking |
| `prepare` | string? | Command to run before each timing run |
| `cleanup` | string? | Command to run after all benchmarking runs |
| `runs` | number? | Number of runs to perform |
| `show-output` | boolean? | Whether to show command output |
| `export-json` | string? | Path to export results as JSON |
| `warmup` | number? | Number of warmup runs |
| `min-runs` | number? | Minimum number of runs |
| `max-runs` | number? | Maximum number of runs |

Note: Fields marked with ? are optional.

## Parameter Lists

The `parameter-list` array allows you to specify multiple parameters that will be substituted in the command. Each parameter has:
- `name`: The parameter name (used in the command with `{name}`)
- `values`: Comma-separated list of values to test

## Error Handling

The wrapper will:
1. Validate the JSON configuration file format
2. Check that hyperfine is available
3. Report any errors from hyperfine execution
4. Exit with non-zero status on any error

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
