use std::io::{BufRead, BufReader};

use longest_common_subsequence::brute_force;
use longest_common_subsequence::dynamic_programming;

fn main() {
    let file = std::fs::File::open("../test_strings.txt").expect("Couldn't open test cases file");
    let file = BufReader::new(file);

    for (i, line) in file.lines().enumerate() {
        let line = line.unwrap();
        let (text1, text2) = line.split_once(' ').unwrap();
        println!(
            "{}: text 1 len: {}. text 2 len: {}",
            i,
            text1.len(),
            text2.len()
        );
        println!(
            "Brute Force: {:?}",
            brute_force::lcs(text1, text2).map(|s| s.len())
        );
        println!(
            "Dynamic Programming: {:?}",
            dynamic_programming::lcs(text1, text2)
        );
        println!()
    }
}
