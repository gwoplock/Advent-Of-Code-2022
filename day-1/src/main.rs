use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if (args.len() < 2){
        println!("requires path to input file");
        return;
    }

    solveProblem(&args[1]);
}

fn solveProblem(inputPath: &String){
    let mut cal_counts: Vec<i32> = Vec::new();
    let mut cal: i32 = 0;
    if let Ok(lines) = read_lines(inputPath) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(val) = line {
                if val == ""{
                    cal_counts.push(cal);
                    cal = 0;
                } else {
                    cal += val.parse::<i32>().unwrap();
                }
            }
        }
        cal_counts.push(cal);
        cal = 0;
    }

    // let mut max_cal_count = 0;

    // for cal_total in cal_counts {
    //     println!("{}", cal_total);
    //     if cal_total > max_cal_count{
    //         max_cal_count = cal_total
    //     }
    // }
    // println!("solution pt1: {}", max_cal_count);

    cal_counts.sort();
    cal_counts.reverse();
    let top_3_cal_total = cal_counts[0] + cal_counts[1] + cal_counts[2];
    
    println!("solution pt1: {}", cal_counts[0]);
    println!("solution pt2: {}", top_3_cal_total)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}