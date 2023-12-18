#![allow(dead_code, unused_mut, unused_variables)]
use std::fs;

#[derive(Debug, Clone)]
pub struct Card {
    index: i32, // card number
    winners: Vec<i32>,
    candidates: Vec<i32>
}

impl Card {
    fn new(line: &str) -> Self {
        let mut parts = line.split(":");
        let index = get_index(parts.next().unwrap());
        let next_part = parts.next().unwrap();
        let mut sub_parts = next_part.split("|");
        let winners = get_numbers(sub_parts.next().unwrap());
        let candidates = get_numbers(sub_parts.next().unwrap());
        
        Self {
            index,
            winners,
            candidates
        }
    }
    fn count_winning_numbers(&self) -> u32 {
        let mut sum: u32 = 0;
        for candidate in &self.candidates {
            if self.winners.contains(candidate) {
                sum += 1;
            }
        }
        return sum;
    }
    fn points(&self) -> i32 {
        let count = self.count_winning_numbers();
        let points = count_to_sum(count);
        return points;
    }
}

fn get_index(parts: &str) -> i32 {
    for smaller_part in parts.split_ascii_whitespace() {
        if smaller_part != "Card" {
            return smaller_part.parse().unwrap();
        }
    }
    return 0;
}

fn get_numbers(line_part: &str) -> Vec<i32> {
    let mut output = vec![];
    for number in line_part.split_ascii_whitespace() {
        output.push(number.parse().unwrap())
    }
    return output;
}

fn count_to_sum(count: u32) -> i32 {
    if count == 1 {
        return 1;
    } else if count > 1 {
        return 2_i32.pow(count - 1)
    }
    return 0;
}

fn main() {
    let mut contents = fs::read_to_string("src/prompt.txt").expect("Error reading prompt.txt");
    part_one(contents.clone());
}

fn part_one(contents: String) {
    let mut sum: i32 = 0;
    for line in contents.lines() {
        let card = Card::new(line);
        sum += card.points()
    }
    println!("{}", sum);
}