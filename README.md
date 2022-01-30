# QOI-Decoder
This is a decoder for the [QOI image format](https://qoiformat.org), written in Rust. Currently the decoder uses nom to parse any .qoi image as byte chunks and converts each pixel to struct [rgb::RGBA](https://docs.rs/rgb/0.8.31/rgb/struct.RGBA.html). These pixels are stored in memory as a Vector.

## Using the decoder
You will need Rust installed on your system to use the decoder. You can do so by using [rustup](https://rustup.rs). Once this is done, you can clone the repository and install the binary as follows.
```bash
$ git clone https://github.com/ayush-sb/QOI-Decoder
$ cargo install -path QOI-Decoder
```
After this, you can use the decoder as shown
```bash
# decode any .qoi file
$ QOI-Decoder ~/dice.qoi
```
