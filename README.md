# fuck-rust
Brainfuck interpreter in Rust

# How to run

The interpreter accepts either a complete program

    cargo run ">++++[-<+>]"


or a file name

    echo ">++++[-<+>]" > program.bf
    cargo run program.bf
