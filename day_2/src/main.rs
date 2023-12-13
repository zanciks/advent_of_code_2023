use std::fs;

fn main() {
    let mut contents = fs::read_to_string("src/prompt.txt").expect("Error reading prompt.txt");
    part_one(contents.clone());
    part_two(contents.clone());
}

fn part_one(contents: String) {
    let mut sum: i32 = 0;
    let lines = contents.split("\n");
    for line in lines {
        let line = replace_colors(line);
        let (index, max_red, max_green, max_blue) = parse_line(line);
        let conditions_met = check_conditions(max_red, max_green, max_blue);
        if conditions_met == true {sum += index as i32}
    }
    println!("{}", sum)
}

fn part_two(contents: String) {
    let mut sum: i32 = 0;
    let lines = contents.split("\n");
    for line in lines {
        let line = replace_colors(line);
        let (index, max_red, max_green, max_blue) = parse_line(line);
        sum += calculate_set_power(max_red, max_green, max_blue);
    }
    println!("{}", sum);
}

fn replace_colors(line: &str) -> String {
    let line = line.replace(" red", "R");
    let line = line.replace(" blue", "B");
    let line = line.replace(" green", "G");
    let line = line.replace("Game ", "");
    let line = line.replace(", ", " ");
    let line = line.replace("; ", " ; ");

    return line.clone();
}

fn parse_line(line: String) -> (u8, u8, u8, u8) {
    let mut max_red: u8 = 0;
    let mut max_blue: u8 = 0;
    let mut max_green: u8 = 0;
    let mut index: u8 = 0;

    let dice = line.split(" ");
    for die in dice {
        for c in die.chars() {
            match c {
                'R' => max_red = max_red.max(die[..die.len()-1].parse().unwrap()),
                'G' => max_green = max_green.max(die[..die.len()-1].parse().unwrap()),
                'B' => max_blue = max_blue.max(die[..die.len()-1].parse().unwrap()),
                ':' => index = die[..die.len()-1].parse().unwrap(),
                _ => ()
            }
        }
    }
    return (index, max_red, max_green, max_blue);
}

fn check_conditions(max_red: u8, max_green: u8, max_blue: u8) -> bool {
    if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
        return true;
    }
    return false;
}

fn calculate_set_power(max_red: u8, max_green: u8, max_blue: u8) -> i32 {
    return max_red as i32 * max_green as i32 * max_blue as i32
}