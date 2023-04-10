# loxide

An implementation of Lox from [Crafting Interpreters](http://craftinginterpreters.com/).

## Current Progress

Token Scanning: ✅
Parsing: _Basic recurisve descent parser implemented for expressions. Currently stops on parsing error_

Currently the compiler will tokenise the source, and then parse a single top-level expression. If it encounters an error it will print it to the screen.

## Building

### Requirements

- Rust 2021 Edition (1.60.0+)
- Cargo (1.60.0+)

### Building

```bash
cargo build
```

## Running

```bash
cargo run
```

## Usage

### Interactive Mode

Running the program without any arguments will start the interactive mode.

```bash
lox >
```

### File Mode

You can also run the program with a file as the first argument.

```bash
cargo run examples/hello_world.lox
```
