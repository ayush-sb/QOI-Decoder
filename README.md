# QOI-Decoder
This is a decoder for the [QOI image format](https://qoiformat.org), written in Rust. Currently the decoder uses nom to parse any .qoi image as byte chunks and converts each pixel to struct [rgb::RGBA](https://docs.rs/rgb/0.8.31/rgb/struct.RGBA.html). These pixels are stored in memory as a Vector.

## Installing Rust
You will need Rust installed on your system to use the decoder. You can do so by using [rustup](https://rustup.rs).

## Using the decoder
Go to /target/release, you can then specify the .qoi file as a command line argument
```bash
$ ./QOI-Decoder ~/dice.qoi
```
