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

fn find_common(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut c = a.clone();
    c.retain(|x| b.contains(x));
    c
}

fn solve_problem(data: String) {
    let part1 = data
        .trim()
        .split("\n")
        .map(|x| {
            let pocket1 = x[..x.len() / 2].as_bytes().to_vec();
            let pocket2 = x[x.len() / 2..].as_bytes().to_vec();

            let common = find_common(&pocket1, &pocket2);

            // dbg!(&common);

            let priority;
            if common[0] <= b'Z' {
                priority = ((common[0] - b'A') + 27) as i64;
            } else if common[0] <= b'z' {
                priority = ((common[0] - b'a') + 1) as i64;
            } else {
                panic!("wat")
            }

            // println!("letter {}, priority {}", common[0], priority);

            priority
        })
        .sum::<i64>();

    let mut i = 0;
    let all_sacks = data.trim().split("\n").collect::<Vec<&str>>();
    // dbg!(&all_sacks);
    let mut badge_pri: Vec<i64> = Vec::new();
    while i < all_sacks.len() {
        let common1 = find_common(
            &all_sacks[i].as_bytes().to_vec(),
            &all_sacks[i + 1].as_bytes().to_vec(),
        );
        let badge = find_common(&common1, &all_sacks[i + 2].as_bytes().to_vec());

        let priority;
        if badge[0] <= b'Z' {
            priority = ((badge[0] - b'A') + 27) as i64;
        } else if badge[0] <= b'z' {
            priority = ((badge[0] - b'a') + 1) as i64;
        } else {
            panic!("wat")
        }

        badge_pri.push(priority);
        i += 3;
    }

    let part2 = badge_pri.iter().sum::<i64>();
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
