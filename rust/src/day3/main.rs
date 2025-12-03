use rust::read_lines;

fn main() {
    let dummy_input: Vec<String> = "987654321111111,811111111111119,234234234234278,818181911112111".split(",").map(|s| s.to_string()).collect();
    let real_input: Vec<String> = read_lines(3);
    println!("{}", part1(real_input.clone()));
    println!("{}", part2(real_input.clone()));
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
fn part2(megavec: Vec<String>) -> u64 {
    let mut max_joltage: u64 = 0;
    for batbank in megavec.iter().map(|s| s.chars().map(|c|c.to_digit(10).unwrap()).collect::<Vec<u32>>()) {
        let mut batbank_max_joltage: u64 = 0;
        let mut start_i = 0;
        for l in 0..12 {
            let batbank_minus:Vec<u32> = batbank[start_i..batbank.len()-(11-l)].to_owned();
            let (d, i) = highest_non_ending_nr(batbank_minus.clone());
            batbank_max_joltage = (batbank_max_joltage*10) + d as u64;
            start_i += i+1;
        }
        max_joltage += batbank_max_joltage;
    }
    max_joltage
}

fn highest_non_ending_nr(batbank: Vec<u32>) -> (u32, usize) {
    let highest = batbank.iter().fold(0, |acc, x| if x > &acc { *x } else {acc});
    (highest,batbank.iter().position(|x1| {*x1 == highest}).unwrap())
}