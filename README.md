# fuck-rust
Brainfuck compiler in Rust

# How to run

The compiler accepts either a complete program

    cargo run ">++++[-<+>]"


or a file name

    echo ">++++[-<+>]" > program.bf
    cargo run program.bf
