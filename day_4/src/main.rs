#![allow(dead_code, unused_mut, unused_variables)]
use std::fs;

fn main() {
    let mut contents = fs::read_to_string("src/prompt.txt").expect("Error reading prompt.txt");
    part_one(contents);
}

fn part_one(contents: String) {
    let mut sum: i32 = 0;
    for line in contents.lines() {
        let mut count: u32 = 0;
        let line = clean_lines(line);
        let mut parts = line.split("|");
        let winners = winners(parts.next().unwrap());
        let candidates = candidates(parts.next().unwrap());
        for candidate in candidates {
            if winners.contains(&candidate){
                count += 1;
            }
        }
        sum += count_to_sum(count);
    }
    println!("{}", sum);
}

fn clean_lines(line: &str) -> &str  {
    let mut parts = line.split(":");
    let first = parts.next();
    return parts.next().unwrap();
}

fn winners(line: &str) -> Vec<i32> {
    let mut winners = vec![];
    for  number in line.split_ascii_whitespace() {
        winners.push(number.parse().unwrap());
    }

    return winners;
}

fn candidates(line: &str) -> Vec<i32> {
    let mut candidates = vec![];

    for number in line.split_ascii_whitespace() {
        candidates.push(number.parse().unwrap());
    }

    return candidates;
}

fn count_to_sum(count: u32) -> i32 {
    if count == 1 {
        return 1;
    } else if count > 1 {
        return 2_i32.pow(count - 1)
    }
    return 0;
}