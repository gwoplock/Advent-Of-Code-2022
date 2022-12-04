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

fn solve_problem(data: String) {
    let start_part_1 = Instant::now();
    let part1: i64 = data
        .trim()
        .split("\n")
        .map(|x| {
            const START: usize = 0;
            const END: usize = 1;
            let jobs = x
                .split(",")
                .map(|y| {
                    y.split("-")
                        .map(|y| y.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>()
                })
                .collect::<Vec<Vec<i64>>>();

            if jobs[0][START] <= jobs[1][START] && jobs[0][END] >= jobs[1][END] {
                return 1;
            } else if jobs[1][START] <= jobs[0][START] && jobs[1][END] >= jobs[0][END] {
                return 1;
            } else {
                return 0;
            }
        })
        .sum();
    let duration_part_1 = start_part_1.elapsed();

    let start_part_2 = Instant::now();
    let part2 = data
        .trim()
        .split("\n")
        .map(|x| {
            const START: usize = 0;
            const END: usize = 1;
            let jobs = x
                .split(",")
                .map(|y| {
                    y.split("-")
                        .map(|y| y.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>()
                })
                .collect::<Vec<Vec<i64>>>();

            if jobs[0][START] <= jobs[1][START] && jobs[0][END] < jobs[1][START] {
                return 0;
            } else if jobs[1][START] <= jobs[0][START] && jobs[1][END] < jobs[0][START] {
                return 0;
            } else {
                return 1;
            }
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
