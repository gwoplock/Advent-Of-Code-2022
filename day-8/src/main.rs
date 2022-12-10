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

fn is_visible(x: usize, y: usize, trees: &Vec<Vec<i64>>, hight: i64) -> bool {
    if x == 0 || y == 0 {
        return true;
    } else if x == trees.len() - 1 || y == trees[x].len() - 1 {
        return true;
    } else {
        let mut check: isize = (x - 1) as isize;
        let mut found_higher = false;
        while check >= 0 {
            if trees[check as usize][y] >= hight {
                found_higher = true;
                break;
            }
            check -= 1;
        }
        if !found_higher {
            return true;
        }

        check = (x + 1) as isize;
        found_higher = false;
        while (check as usize) < trees.len() {
            if trees[check as usize][y] >= hight {
                found_higher = true;
                break;
            }
            check += 1;
        }
        if !found_higher {
            return true;
        }

        check = (y - 1) as isize;
        found_higher = false;
        while check >= 0 {
            if trees[x][check as usize] >= hight {
                found_higher = true;
                break;
            }
            check -= 1;
        }
        if !found_higher {
            return true;
        }

        check = (y + 1) as isize;
        found_higher = false;
        while (check as usize) < trees[x].len() {
            if trees[x][check as usize] >= hight {
                found_higher = true;
                break;
            }
            check += 1;
        }
        if !found_higher {
            return true;
        }

        return false;
    }
}

fn get_view_dist(x: usize, y: usize, trees: &Vec<Vec<i64>>) -> i64 {
    let hight = trees[x][y];

    if x == 0 || y == 0 {
        return 0;
    } else if x == trees.len() - 1 || y == trees[x].len() - 1 {
        return 0;
    } else {
        let mut check: isize = (x - 1) as isize;
        let mut count_up = 0;
        while check >= 0 {
            count_up += 1;
            if trees[check as usize][y] >= hight {
                break;
            }
            check -= 1;
        }

        check = (x + 1) as isize;
        let mut count_down = 0;
        while (check as usize) < trees.len() {
            count_down += 1;
            if trees[check as usize][y] >= hight {
                break;
            }
            check += 1;
        }

        check = (y - 1) as isize;
        let mut count_left = 0;
        while check >= 0 {
            count_left += 1;
            if trees[x][check as usize] >= hight {
                break;
            }
            check -= 1;
        }

        check = (y + 1) as isize;
        let mut count_right = 0;
        while (check as usize) < trees[x].len() {
            count_right += 1;
            if trees[x][check as usize] >= hight {
                break;
            }
            check += 1;
        }

        return count_down * count_left * count_right * count_up;
    }
}

fn solve_problem(data: String) {
    let trees = data
        .trim()
        .split("\n")
        .map(|x| {
            x.trim()
                .to_string()
                .chars()
                .map(|x| x.to_digit(10).unwrap() as i64)
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    dbg!(&trees);

    let start_part_1 = Instant::now();

    let mut x: usize = 0;
    let mut part1 = 0;
    while x < (&trees).len() {
        let mut y: usize = 0;
        while y < (&trees[x]).len() {
            if is_visible(x, y, &trees, (&trees)[x][y]) {
                part1 += 1;
            }
            y += 1;
        }
        x += 1;
    }

    let duration_part_1 = start_part_1.elapsed();

    let start_part_2 = Instant::now();

    let mut x: usize = 0;
    let mut view_dist: Vec<i64> = Vec::new();
    while x < (&trees).len() {
        let mut y: usize = 0;
        while y < (&trees[x]).len() {
            view_dist.push(get_view_dist(x, y, &trees));
            y += 1;
        }
        x += 1;
    }

    dbg!(&view_dist);

    let part2 = view_dist.iter().fold(std::i64::MIN, |a, b| a.max(*b));

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
