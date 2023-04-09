# loxide

An implementation of Lox from [Crafting Interpreters](http://craftinginterpreters.com/).

## Current Progress

Begun implementation of token scanning; given lox source will print out the tokens.
Has both an interactive prompt, and a file input mode.

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
