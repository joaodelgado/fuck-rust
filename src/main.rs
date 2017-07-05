use std::env;
use std::io;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn match_jumps(program: &String) -> HashMap<usize, usize> {
    let mut stack = Vec::new();
    let mut matches_map: HashMap<usize, usize> = HashMap::new();

    for (i, c) in program.chars().enumerate() {
        match c {
            '[' => stack.push(i),
            ']' => {
                let other = stack.pop().expect("Non matching parenthesis");
                matches_map.insert(other, i);
                matches_map.insert(i, other);
            }
            _ => {}
        }
    }

    if stack.len() > 0 {
        panic!("Non matching parenthesis");
    }

    matches_map
}

fn main() {
    let mut program = env::args()
                          .nth(1)
                          .expect("Please provide a program or file name as the first argument.");

    // Read program from file
    if Path::new(&program).exists() {
        let mut f = File::open(&program).expect("Error opening file");
        program.clear();
        let read = f.read_to_string(&mut program).expect("Error reading program from file");
        if read == 0 {
            panic!("Read 0 bytes")
        }
    }

    let mut mem: Vec<u8> = vec![0; 30000]; // memory vector
    let mut ip = 0; // pointer for the current instruction char
    let mut mp = 0; // pointer for the current memory index

    let jumps = match_jumps(&program);

    while ip < program.len() {
        let c = program.chars().nth(ip).unwrap();
        match c {
            '+' => mem[mp] = mem[mp].wrapping_add(1),
            '-' => mem[mp] = mem[mp].wrapping_sub(1),
            '>' => mp += 1,
            '<' => mp -= 1,
            '[' => {
                if mem[mp] == 0 {
                    ip = *jumps.get(&ip).unwrap();
                }
            }
            ']' => {
                ip = *jumps.get(&ip).unwrap();
                ip -= 1;
            }
            '.' => print!("{}", mem[mp] as char),
            ',' => {
                let mut buf = [0u8];
                io::stdin().read_exact(&mut buf).expect("Error reading from stdin");
                mem[mp] = buf[0];
            }
            _ => {}
        }
        ip += 1;
    }

}
