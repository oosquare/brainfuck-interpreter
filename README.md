## brainfuck-interpreter

### Overview

A fast, powerful and configurable interpreter written in Rust,
which allows various options to meet different demends, including
memory (tape) length configuration, EOF handling configuration and
so on.

### Usage

#### Quick Start

```sh
$ git clone https://github.com/ctj12461/brainfuck-interpreter.git
$ cd brainfuck-interpreter
$ cargo install --path . # The program will be installed to ~/.cargo/bin
$ brainfuck-interpreter ./examples/helloworld.bf
Hello World!
```

#### Verbose Version

```plain
Usage: brainfuck-interpreter [OPTIONS] <SOURCE>

Arguments:
  <SOURCE>
          the path of the brainfuck program source code file.

Options:
      --len <LEN>
          the memory (tape) length the program will use.

          [default: 32768]

      --addr <ADDR>
          the address range of the memory (tape).

           - unsigned: [0, len - 1]
           - signed: [-ceil(len / 2), ceil(len / 2) - 1]

          [default: unsigned]
          [possible values: unsigned, signed]

      --cell <CELL>
          the data type of one cell in the memory (tape).

          [default: int8]
          [possible values: int8, int32]

      --overflow <OVERFLOW>
          the operation the interpreter should do when an overflow error occurs.

           - wrap: automatically wrap the value in cell (e.g.: `127 + 1` => `-127`)
           - error: throw an error and abort

          [default: wrap]
          [possible values: wrap, error]

      --eof <EOF>
          the operation the interpreter should do when an `EOF` is read.

           - zero: turn `EOF` to `0`
           - keep: keep what the `EOF` is and return it (`EOF == -1`)
           - ignore: ignore this input and leave the cell unchanged

          [default: ignore]
          [possible values: zero, keep, ignore]

      --input <INPUT>
          the input stream type.

          [default: std]
          [possible values: null, std]

      --output <OUTPUT>
          the output stream type.

          [default: char-std]
          [possible values: char-std, int-std]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### License

Licensed under MIT.

Copyright (C) 2023 Justin Chen (ctj12461)

