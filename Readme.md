# JSON2CBOR

This repo contains two really simple programs to convert JSON to CBOR and vice versa. It does the transformation using [`serde-transcode`](https://github.com/sfackler/serde-transcode).

## Install

1. [Install Rust](https://rustup.rs/).
2. `cargo install json2cbor`.

## Use

    json2cbor in.json out.cbor
    cbor2json in.cbor out.json

That's it!
