#![allow(dead_code)]

use std::fs;

fn main() {
    let mut contents = fs::read_to_string("src/prompt.txt").expect("Error reading prompt.txt");
    println!("{}", part_two(&mut contents));
    println!("{}", part_one(&mut contents));
}

fn part_two(contents: &mut String) -> i32 {
    let mut sum = 0;
    let lines = contents.split("\n");
    for line in lines {
        let line = line.replace("one", "o1e");
        let line = line.replace("two", "t2o");
        let line = line.replace("three", "thr3e");
        let line = line.replace("four", "fo4r");
        let line = line.replace("five", "fi5e");
        let line = line.replace("six", "s6x");
        let line = line.replace("seven", "se7en");
        let line = line.replace("eight", "ei8ht");
        let line = line.replace("nine", "ni9e");
        sum += parse_line(&line);
    }
    return sum;
}


fn part_one(contents: &mut String) -> i32 {
    let mut sum = 0;
    let lines = contents.split("\n");
    for line in lines {
        sum += parse_line(line);
    }
    return sum;
}

fn parse_line(line: &str) -> i32 {
    let mut sum = 0;
    for c in line.chars() {
        let char_value = match_char(&c);
        if char_value != 0 {
            sum += 10 * char_value;
            break;
        }
    }
    for c in line.chars().rev() {
        let char_value = match_char(&c);
        if char_value != 0 {
            sum += char_value;
            break;
        }
    }
    return sum;
}

fn match_char(c: &char) -> i32 {
    match c {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
       _ => 0,
   }
}