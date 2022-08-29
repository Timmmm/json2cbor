# JSON2CBOR

This repo contains two really simple programs to convert JSON to CBOR and vice versa. It does the transformation using [`serde-transcode`](https://github.com/sfackler/serde-transcode).

## Install

1. [Install Rust](https://rustup.rs/).
2. `cargo install json2cbor`.

## Use

`json2cbor` and `cbor2json` both take the same flags & arguments

### Reads/writes to stdin/stdout by default

```sh
echo '{"foo":"bar"}' | json2cbor | cbor2json
# Output: {"foo":"bar"}
```

### Read from an input file

```sh
json2cbor -i in.json
```

### Write to an output file

```sh
json2cbor -o out.cbor
```

### Read & write at the same time

```sh
json2cbor -i in.json -o out.cbor
```

### Get help

```sh
json2cbor --help
```
