use rust::read_lines;

fn main() {
    let dummy_input: Vec<String> = "987654321111111,811111111111119,234234234234278,818181911112111".split(",").map(|s| s.to_string()).collect();
    let real_input: Vec<String> = read_lines(3);
    println!("{}", part1(real_input.clone()));
    println!("{}", part2(real_input.clone()));
}

fn part1(megavec: Vec<String>) -> u64 {
    highest_substring(megavec,2)
}
fn part2(megavec: Vec<String>) -> u64 {
    highest_substring(megavec,12)
}

fn highest_non_ending_nr(numvec: Vec<u32>) -> (u32, usize) {
    let highest = numvec.iter().fold(0, |acc, x| if x > &acc { *x } else {acc});
    (highest, numvec.iter().position(|x1| {*x1 == highest}).unwrap())
}

fn highest_substring(megavec: Vec<String>, string_length: usize) -> u64 {
    let mut total_highest: u64 = 0;
    for numvec in megavec.iter().map(|s| s.chars().map(|c|c.to_digit(10).unwrap()).collect::<Vec<u32>>()) {
        let mut numvec_highest: u64 = 0;
        let mut start_i = 0;
        for l in 0..string_length {
            let sliced_numvec:Vec<u32> = numvec[start_i..numvec.len()-((string_length-1)-l)].to_owned();
            let (d, i) = highest_non_ending_nr(sliced_numvec);
            numvec_highest = (numvec_highest *10) + d as u64;
            start_i += i+1;
        }
        total_highest += numvec_highest;
    }
    total_highest
}