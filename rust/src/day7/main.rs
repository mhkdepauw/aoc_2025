use std::collections::{HashMap, HashSet};
use rust::read_lines;
use std::fs::read_to_string;

fn main() {
    let dummy_input: Vec<String> = read_to_string("src/day7/dummy_input.txt")
        .expect("Failed to get file as string")
        .lines()
        .map(|l| l.to_string())
        .collect();
    let real_input: Vec<String> = read_lines(7);
    println!("{}", part1(&real_input));
    println!("{}", part2(&real_input));
}

fn part1(megavec: &Vec<String>) -> i32 {
    let mut split_count: i32 = 0;
    let megavec_chars: Vec<Vec<char>> = megavec.iter().map(|s| s.chars().collect()).collect();
    let mut beams_js: HashSet<usize> = HashSet::from([megavec_chars[0].iter().position(|s| s.eq(&'S')).unwrap()]);
    for i in 0..megavec_chars.len()-1 {
        let mut new_beamsjs: HashSet<usize> = HashSet::new();
            for j in beams_js {
                if megavec_chars[i+1][j] == '^' {
                    new_beamsjs.insert(j-1);
                    new_beamsjs.insert(j+1);
                    split_count += 1;
                }
                else { new_beamsjs.insert(j); }
            }
        beams_js = new_beamsjs;
    }
    split_count
}

fn part2(megavec: &Vec<String>) -> i64 {
    let megavec_chars: Vec<Vec<char>> = megavec.iter().map(|s| s.chars().collect()).collect();
    let mut beams_js: HashMap<usize, i64> = HashMap::from([(megavec_chars[0].iter().position(|s| s.eq(&'S')).unwrap(), 1)]);
    for i in 0..megavec_chars.len()-1 {
        let mut new_beamsjs: HashMap<usize, i64> = HashMap::new();
        for (j,tms) in beams_js {
            if megavec_chars[i+1][j] == '^' {
                *new_beamsjs.entry(j-1).or_insert(0) += tms;
                *new_beamsjs.entry(j+1).or_insert(0) += tms;
            }else {
                *new_beamsjs.entry(j).or_insert(0) += tms;
            }
        }
        beams_js = new_beamsjs;
    }
    beams_js.values().copied().sum::<i64>()
}
