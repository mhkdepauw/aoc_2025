use std::ops::Index;
use std::str::Chars;
use rust::read_lines;

fn main() {
    let dummy_input: Vec<String> = "987654321111111,811111111111119,234234234234278,818181911112111".split(",").map(|s| s.to_string()).collect();
    let real_input: Vec<String> = read_lines(3);
    println!("{}", part1(real_input.clone()));
    // println!("{}", part2(dummy_input.clone()));
}

fn part1(megavec: Vec<String>) -> u32 {
    let mut max_joltage: u32 = 0;
    for batbank in megavec.iter().map(|s| s.chars().map(|c|c.to_digit(10).unwrap()).collect::<Vec<u32>>()) {
        let batbank_minus:Vec<u32> = batbank[..batbank.len()-1].to_owned();
        let (d1, i1 ) = highest_non_ending_nr(batbank_minus);
        let (d2, _) = highest_non_ending_nr(batbank[i1 + 1..].to_owned());
        max_joltage += (d1*10) + d2;
    }
    max_joltage
}
// TODO: check for 12 best numbers by keeping in mind how many number there will be after the first pick, limit picks everytime
fn part2(megavec: Vec<String>) -> i32 {
    return 0;
}

fn highest_non_ending_nr(batbank: Vec<u32>) -> (u32, usize) {
    let highest = batbank.iter().fold(0, |acc, x| if x > &acc { *x } else {acc});
    (highest,batbank.iter().position(|x1| {*x1 == highest}).unwrap())
}