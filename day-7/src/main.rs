use std::{env, result};
use std::fs::File;
use std::io::{self, Read};
use std::ops::Try;
use std::path::Path;
use std::time::Instant;
use trees::Tree;

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
    let start_part_1 = Instant::now();

    let part_1_iter = data.trim().split("\n").peekable(); //skip `cd /`
    part_1_iter.next();
    let mut file_system = Tree::new("/");

    let parse = ||-> Result<(), ()> {
        for x in part_1_iter {
            if x.starts_with("$ ls") {
                while !part_1_iter.peek().unwrap()?.starts_with("$") {
                    
                }
            }
        }
        Ok(())
    }

    let duration_part_1 = start_part_1.elapsed();

    let start_part_2 = Instant::now();
    let part2 = data
        .trim()
        .split("\n")
        .map(|x| {
            return 1;
        })
        .sum::<i64>();
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
