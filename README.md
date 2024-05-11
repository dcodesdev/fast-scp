# Fast SCP

A Rust CLI tool to copy files from your linux remote server to your local machine. Each file is ran on a separate thread, which makes it much faster than the traditional `scp` command.

## Installation

Install the CLI using cargo:

```bash
cargo install fast-scp
```

## Example usage

```bash
fast-scp receive <remote-path> <local-path> --host <host> --user <username> --private-key [path-to-private-key]
```

## Commands

### Receive

The command to receive files from the linux remote server ot the local machine.

#### Arguments

| Name                                  | Description                                         | Required |
| ------------------------------------- | --------------------------------------------------- | -------- |
| `<remote-path>`                       | Path to the **file or directory** on the server.    | Yes      |
| `<local-path>`                        | The path to the **directory** on the local machine. | Yes      |
| `--host <host>`                       | The IP address of the remote server.                | Yes      |
| `--user <username>`                   | The username to login to the remote server.         | Yes      |
| `--private-key [path-to-private-key]` | The path to the private key to authenticate with.   | No       |

## License

Licensed under MIT License. See [LICENSE](LICENSE) for more information.
