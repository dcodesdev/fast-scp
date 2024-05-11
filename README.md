# Fast SCP

A Rust CLI tool to copy files from your linux remote server to your local machine. Each file is ran on a separate thread, which makes it much faster than the traditional `scp` command.

## Example

```bash
fast-scp receive <remote-path> <local-path> --host <host> --user <username> --private-key [path-to-private-key]
```

## License

Licensed under MIT License. See [LICENSE](LICENSE) for more information.
