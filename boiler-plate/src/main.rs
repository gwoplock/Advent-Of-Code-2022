use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("requires path to input file");
        return;
    }

    if let Ok(lines) = read_lines(&args[1]) {
        solve_problem(lines);
    }
}

fn solve_problem(lines: io::Lines<io::BufReader<File>>){
    for line in lines {
        if let Ok(val) = line {
        }
    }



    println!("solution pt1: {}", );
    println!("solution pt2: {}", );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}