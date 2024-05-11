# scp-rs

**WARNING: THIS CLI TOOL IS STILL IN BETA AND NOT READY FOR USE**

A Rust CLI tool to copy files from remote server to local machine or the other way around, handles tasks concurrently, which makes it faster than the traditional scp command.

## Example

```bash
fast-scp receive <remote-path> <local-path> --host <host> --user <username> --private-key [path-to-private-key]
```

## License

Licensed under MIT License. See [LICENSE](LICENSE) for more information.
