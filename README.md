# KekUpload server

A backend providing a HTTP REST like interface for uploading files written in [rust](https://www.rust-lang.org/).

<br>

[API Documentation](https://oss.kotw.dev/uploadserver/docs/API)

<br>

## License
This project is licensed under the [Mit License](https://mit-license.org/)

<hr>
<br>

## Features
1. Rest api
2. Chunked uploading
3. Embeds

<hr>
<br>

## Clients
- [KekUpload client (svelte)](https://github.com/KotwOSS/kekupload-client)

<br>

You have created your own client for [KekUpload server](https://oss.kotw.dev/kekupload-server)? Great! Just submit an client request in the issues tab of github or write me an email at [kekontheworld@gmail.com](mailto:kekontheworld@gmail.com).

<hr>
<br>

## Usage

### Prerequirements

- Rust Nightly <br>
You need [rustup](https://rustup.rs/) to run this.

```sh
rustup default nightly
```

<br>

### Configuration
Copy default.env to .env and change the settings in .env.

<br>

### Building
```sh
cargo build --release
```

The executable will be located at `target/release/uploadserver`

<br>

### Testing
If you are developing and don't want to rebuild and run the client to release mode use
```sh
cargo run
```
<hr>
<br>

## Goals

- Image compression

<br>

If you have aditional ideas how to make this tool better please create a feature request in the issues tab.

<hr>
<br>

## Contributing
More information [here](https://oss.kotw.dev/kekupload-server/CONTRIBUTE).