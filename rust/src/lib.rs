use std::fs;

pub fn read_lines(day: i32) -> Vec<String> {
    fs::read_to_string(format!("src/day{}/input.txt", day))
        .expect("Failed to get file as string")
        .lines()
        .map(|l| l.to_string())
        .collect()
}
