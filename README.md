# Encodex
![example workflow](https://github.com/grqphical07/encodex/actions/workflows/rust.yml/badge.svg)

A simple CLI tool to encode text

## Installation

Clone this repo then run
```bash
$ cargo install
```

## Usage

To encode text into Base64 run:
```bash
$ encodex "Hello World" -e base64
```

It should print
```SGVsbG8gV29ybGQ=```

To decode values just add the flag ```-d```
```bash
$ encodex "SGVsbG8gV29ybGQ=" -e base64 -d
```

It should print
```Hello World```

## Encoding formats

Currently encodex supports 4 formats

| **Format**     | **Name**   | **Example (Encoded the phrase "Hello World")**                                  |
|-------------|--------|----------------------------------------------------------------------------------------|
| Base64      | Base64 | SGVsbG8gV29ybGQ=                                                                       |
| Binary      | binary | 1001000 1100101 1101100 1101100 1101111 100000 1010111 1101111 1110010 1101100 1100100 |
| Bytes       | bytes  | 72 101 108 108 111 32 87 111 114 108 100                                               |
| Hexadecimal | hex    | 48656c6c6f20576f726c64                                                                 |
