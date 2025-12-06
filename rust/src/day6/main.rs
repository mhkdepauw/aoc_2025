use std::fs;
use std::fs::read_to_string;

fn main() {
    let dummy_input: Vec<String> = read_to_string("src/day6/dummy_input.txt")
        .expect("Failed to get file as string")
        .lines()
        .map(|l| l.to_string())
        .collect();
    println!("{}", part1(&dummy_input));
    println!("{}", part2(&dummy_input));
}

fn part1(megavec: &Vec<String>) -> i32 {
    0
}

fn part2(megavec: &Vec<String>) -> i32 {
    0
}

fn to_op_list(megavec: Vec<String>) -> (Vec<String>, Vec<Vec<i32>>) {
    let processed: Vec<Vec<String>> = megavec
        .iter()
        .map(|s| {
            s.split(" ")
                .filter(|s1| !s1.is_empty())
                .map(|s2| s2.to_string())
                .collect::<Vec<String>>()
        })
        .collect();
    let mut transposed_m: Vec<Vec<i32>> = vec![];
    for i in 0..processed[0].len() {
        let mut transposed: Vec<i32> = vec![];
        for j in 0..processed.len() - 1 {
            transposed.push(processed[j][i].parse().unwrap());
        }
        transposed_m.push(transposed);
    }
    (processed[processed.len() - 1].clone(), transposed_m)
}
