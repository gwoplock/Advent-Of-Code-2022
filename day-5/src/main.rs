use scanf::sscanf;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::time::Instant;
use std::{env, vec};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("requires path to input file");
        return;
    }

    if let Ok(data) = read_file(&args[1]) {
        solve_problem(data);
    }
}

fn solve_problem(data: String) {
    // let mut stack = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
    /*
            [J]         [B]     [T]
            [M] [L]     [Q] [L] [R]
            [G] [Q]     [W] [S] [B] [L]
    [D]     [D] [T]     [M] [G] [V] [P]
    [T]     [N] [N] [N] [D] [J] [G] [N]
    [W] [H] [H] [S] [C] [N] [R] [W] [D]
    [N] [P] [P] [W] [H] [H] [B] [N] [G]
    [L] [C] [W] [C] [P] [T] [M] [Z] [W]
    1   2   3   4   5   6   7   8   9
    */
    let mut stack = vec![
        vec!['L', 'N', 'W', 'T', 'D'],
        vec!['C', 'P', 'H'],
        vec!['W', 'P', 'H', 'N', 'D', 'G', 'M', 'J'],
        vec!['C', 'W', 'S', 'N', 'T', 'Q', 'L'],
        vec!['P', 'H', 'C', 'N'],
        vec!['T', 'H', 'N', 'D', 'M', 'W', 'Q', 'B'],
        vec!['M', 'B', 'R', 'J', 'G', 'S', 'L'],
        vec!['Z', 'N', 'W', 'G', 'V', 'B', 'R', 'T'],
        vec!['W', 'G', 'D', 'N', 'P', 'L'],
    ];
    let start_part_1 = Instant::now();
    data.trim().split("\n").for_each(|x| {
        // dbg!(&x);
        let mut count: usize = 0;
        let mut source: usize = 0;
        let mut dest: usize = 0;
        if let Err(error) = sscanf!(x, "move {} from {} to {}", count, source, dest) {
            panic!("Error {} using sscanf!", error);
        }

        let source_stack = &mut stack[source - 1];
        let mut moved: Vec<_> = source_stack.drain(source_stack.len() - count..).collect();
        moved.reverse();
        stack[dest - 1].append(&mut moved);
        // dbg!(&stack);
    });

    // dbg!(&stack);

    let part1 = stack
        .iter()
        .map(|x| {
            return x.last().unwrap();
        })
        .collect::<String>();
    let duration_part_1 = start_part_1.elapsed();

    // stack = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
    stack = vec![
        vec!['L', 'N', 'W', 'T', 'D'],
        vec!['C', 'P', 'H'],
        vec!['W', 'P', 'H', 'N', 'D', 'G', 'M', 'J'],
        vec!['C', 'W', 'S', 'N', 'T', 'Q', 'L'],
        vec!['P', 'H', 'C', 'N'],
        vec!['T', 'H', 'N', 'D', 'M', 'W', 'Q', 'B'],
        vec!['M', 'B', 'R', 'J', 'G', 'S', 'L'],
        vec!['Z', 'N', 'W', 'G', 'V', 'B', 'R', 'T'],
        vec!['W', 'G', 'D', 'N', 'P', 'L'],
    ];
    let start_part_2 = Instant::now();
    data.trim().split("\n").for_each(|x| {
        // dbg!(&x);
        let mut count: usize = 0;
        let mut source: usize = 0;
        let mut dest: usize = 0;
        if let Err(error) = sscanf!(x, "move {} from {} to {}", count, source, dest) {
            panic!("Error {} using sscanf!", error);
        }

        let source_stack = &mut stack[source - 1];
        let mut moved: Vec<_> = source_stack.drain(source_stack.len() - count..).collect();
        // moved.reverse();
        stack[dest - 1].append(&mut moved);
        // dbg!(&stack);
    });

    let part2 = stack
        .iter()
        .map(|x| {
            return x.last().unwrap();
        })
        .collect::<String>();
    let duration_part_2 = start_part_2.elapsed();

    println!("solution pt1: {} in {:?}", part1, duration_part_1);
    println!("solution pt2: {} in {:?}", part2, duration_part_2);
}

fn read_file<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut file = File::open(filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}
