use std::fs;

pub fn read_lines(day: i32) -> Vec<String> {
    fs::read_to_string(format!("src/day{}/input.txt", day))
        .expect("Failed to get file as string")
        .lines()
        .map(|l| l.to_string())
        .collect()
}
pub fn read_str_separated_by(day: i32, patt: &str) -> Vec<String> {
    fs::read_to_string(format!("src/day{}/input.txt", day))
        .expect("Failed to get file as string")
        .split(patt)
        .map(|s| s.to_string())
        .collect()
}
