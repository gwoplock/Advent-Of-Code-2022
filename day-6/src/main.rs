use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::time::Instant;

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

fn contains_dups(to_check: &[char]) -> bool {
    let mut seen: HashMap<&char, bool> = HashMap::new();
    seen.insert(&'\0', true);
    for x in to_check {
        if seen.get(x) != None {
            return true;
        }
        seen.insert(x, true);
    }

    return false;
}

fn solve_problem(data: String) {
    let start_part_1 = Instant::now();
    let mut circ_buff: [char; 4] = ['\0', '\0', '\0', '\0'];
    let mut circ_buff_index = 0;
    let mut part1: usize = 0;

    for x in data.chars() {
        circ_buff[circ_buff_index] = x;
        part1 += 1;
        circ_buff_index += 1;
        if circ_buff_index > 3 {
            circ_buff_index = 0
        }

        if !contains_dups(&circ_buff) {
            break;
        }
    }

    let duration_part_1 = start_part_1.elapsed();

    let start_part_2 = Instant::now();
    let mut circ_buff_part2: [char; 14] = [
        '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
    ];
    circ_buff_index = 0;
    let mut part2: usize = 0;

    for x in data.chars() {
        circ_buff_part2[circ_buff_index] = x;
        part2 += 1;
        circ_buff_index += 1;
        if circ_buff_index > 13 {
            circ_buff_index = 0
        }

        if !contains_dups(&circ_buff_part2) {
            break;
        }
    }
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
