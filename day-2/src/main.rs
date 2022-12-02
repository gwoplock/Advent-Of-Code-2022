use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

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
    let part1 = data
        .trim()
        .split("\n")
        .map(|x| {
            let hand = x.trim().split(" ").collect::<Vec<&str>>();
            let shape_pts;
            match hand[1] {
                "X" => shape_pts = 1, //rock
                "Y" => shape_pts = 2, //paper
                "Z" => shape_pts = 3, //scissors
                _ => panic!("bad char {} in player's hand", hand[1]),
            }
            let mut win_pts = 0;
            if hand[0] == "A" {
                //rock
                if hand[1] == "Y" {
                    win_pts = 6;
                } else if hand[1] == "Z" {
                    win_pts = 0;
                } else if hand[1] == "X" {
                    win_pts = 3;
                }
            } else if hand[0] == "B" {
                //paper
                if hand[1] == "X" {
                    win_pts = 0;
                } else if hand[1] == "Z" {
                    win_pts = 6;
                } else if hand[1] == "Y" {
                    win_pts = 3;
                }
            } else if hand[0] == "C" {
                //scissors
                if hand[1] == "X" {
                    win_pts = 6;
                } else if hand[1] == "Y" {
                    win_pts = 0;
                } else if hand[1] == "Z" {
                    win_pts = 3;
                }
            } else {
                panic!("bad char {} in openent's hand", hand[1])
            }
            shape_pts + win_pts
        })
        .sum::<i64>();

    let part2 = data
        .trim()
        .split("\n")
        .map(|x| {
            let scissors_pts = 3;
            let paper_pts = 2;
            let rock_pts = 1;

            let hand = x.trim().split(" ").collect::<Vec<&str>>();

            let shape_pts;
            let win_pts;

            if hand[1] == "X" {
                //lose
                win_pts = 0;
                if hand[0] == "A" {
                    //rock
                    shape_pts = scissors_pts;
                } else if hand[0] == "B" {
                    //paper
                    shape_pts = rock_pts;
                } else if hand[0] == "C" {
                    //scissors
                    shape_pts = paper_pts;
                } else {
                    panic!("bad char {} in openent's hand", hand[0])
                }
            } else if hand[1] == "Y" {
                //draw
                win_pts = 3;
                if hand[0] == "A" {
                    //rock
                    shape_pts = rock_pts;
                } else if hand[0] == "B" {
                    //paper
                    shape_pts = paper_pts;
                } else if hand[0] == "C" {
                    //scissors
                    shape_pts = scissors_pts;
                } else {
                    panic!("bad char {} in openent's hand", hand[0])
                }
            } else if hand[1] == "Z" {
                //win
                win_pts = 6;
                if hand[0] == "A" {
                    //rock
                    shape_pts = paper_pts;
                } else if hand[0] == "B" {
                    //paper
                    shape_pts = scissors_pts;
                } else if hand[0] == "C" {
                    //scissors
                    shape_pts = rock_pts;
                } else {
                    panic!("bad char {} in openent's hand", hand[0])
                }
            } else {
                panic!("bad char {} in player's hand", hand[1])
            }

            shape_pts + win_pts
        })
        .sum::<i64>();

    println!("solution pt1: {}", part1);
    println!("solution pt2: {}", part2);
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
