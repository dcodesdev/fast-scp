# Fast SCP

A Rust CLI tool to copy files from your linux remote server to your local machine. Each file is ran on a separate thread, which makes it much faster than the traditional `scp` command.

## Example

```bash
fast-scp receive <remote-path> <local-path> --host <host>
```

### Parameters

- `remote-path`: The path to the file or directory on the remote server.
- `local-path`: The path to the directory on the local machine where the files will be copied to.
- `host`: The IP address or hostname of the remote server.
- `user` (optional): The username to use to connect to the remote server. Default is `root`.
- `private-key` (optional): The path to the private key to use to connect to the remote server. Default is `~/.ssh/id_rsa`.
- `replace` (optional): If set, the local file will be replaced if it already exists. Default is `false`.

## License

Licensed under MIT License. See [LICENSE](LICENSE) for more information.
