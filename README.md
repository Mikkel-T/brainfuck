# Brainfuck

A command line tool for the brainfuck language written in Rust. Can both check, minify and run brainfuck code.

## Commands

### Run

Runs brainfuck code from a file.

### Minify

Minifies a brainfuck file by removing everything that isn't one of the 8 valid instructions (`>`, `<`, `+`, `-`, `.`, `,`, `[`, `]`).

### Check

Checks the brainfuck code for any syntax errors (Loop braces not balanced). This is also run automatically before running the code.
