# Upload Server used by KekUpload

## Features
1. Image compression (because i have shitty internet)
2. Rest api
3. Chunked uploading
4. Embeds

<hr>
<br>

## Can i use this bullshit?
Well yeah its open source dumbass.

<hr>
<br>

## How do i use this?

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

<hr>
<br>

## Contributing
More information [here](https://github.com/KekOnTheWorld/uploadserver/blob/main/CONTRIBUTE.md).