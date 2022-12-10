use scanf::sscanf;
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

fn add_maybe(clock: i64, multiple: &mut i64, acc: i64) -> i64 {
    if clock == 20 + (*multiple * 40) {
        *multiple += 1;
        let strength = clock * acc;
        return strength;
    }
    return 0;
}

fn print_maybe(clock: i64, acc: i64) {
    let hscan = clock % 40;
    if hscan == 0 {
        println!("");
    }
    if hscan >= acc - 1 && hscan <= acc + 1 {
        print!("#");
    } else {
        print!(" ");
    }
}

fn solve_problem(data: String) {
    let start_part_1 = Instant::now();

    let instructions: Vec<&str> = data.trim().split("\n").collect();

    let mut part1: i64 = 0;
    {
        let mut acc: i64 = 1;
        let mut clock: i64 = 0;
        let mut multiple: i64 = 0;

        for x in &instructions {
            if x.starts_with("noop") {
                clock += 1;
                part1 += add_maybe(clock, &mut multiple, acc);
            } else if x.starts_with("addx") {
                let mut inc: i64 = 0;
                if let Err(error) = sscanf!(x, "addx {}", inc) {
                    panic!("Error {} using sscanf!", error);
                }
                dbg!(inc);
                clock += 1;
                part1 += add_maybe(clock, &mut multiple, acc);
                clock += 1;
                part1 += add_maybe(clock, &mut multiple, acc);
                acc += inc;
            }
        }
    }

    let duration_part_1 = start_part_1.elapsed();

    let start_part_2 = Instant::now();

    {
        let mut acc: i64 = 1;
        let mut clock: i64 = 0;

        for x in &instructions {
            if x.starts_with("noop") {
                print_maybe(clock, acc);
                clock += 1;
            } else if x.starts_with("addx") {
                let mut inc: i64 = 0;
                if let Err(error) = sscanf!(x, "addx {}", inc) {
                    panic!("Error {} using sscanf!", error);
                }
                // dbg!(inc);
                print_maybe(clock, acc);
                clock += 1;
                print_maybe(clock, acc);
                clock += 1;
                acc += inc;
            }
        }
    }

    let duration_part_2 = start_part_2.elapsed();

    println!("solution pt1: {} in {:?}", part1, duration_part_1);
    println!("solution pt2: in {:?}", duration_part_2);
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
