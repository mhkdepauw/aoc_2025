use rust::read_str_separated_by;
use std::collections::HashSet;

fn main() {
    let dummy_input: Vec<String> = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
        .split(",").map(|s|s.to_string()).collect();
    let real_input = read_str_separated_by(2, ",");
    println!("{}", part1(&real_input));
    println!("{}", part2(&real_input));
}

fn part1(megavec: &Vec<String>) -> i64 {
    let mut invalid_sum: i64 = 0;
    for range in megavec {
        let (left, right): (i64, i64) = {
            let mut it = range.split("-").map(|s| s.parse().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        };
        let mut lbound: i64 = {
            let left_length = left.to_string().chars().count();
            if left_length % 2 == 0 {
                left.to_string()[..(left_length / 2)].parse().unwrap()
            } else {
                10_i64.pow((left_length / 2) as u32)
            }
        };
        let mut pot_inv: i64 = lbound.to_string().repeat(2).parse().unwrap();
        while pot_inv <= right {
            if pot_inv >= left {
                invalid_sum += pot_inv;
            }
            lbound += 1;
            pot_inv = lbound.to_string().repeat(2).parse().unwrap();
        }
    }
    invalid_sum
}

fn part2(megavec: &Vec<String>) -> i64 {
    let mut invalid_sum: i64 = 0;
    for range in megavec {
        let (left, right): (i64, i64) = {
            let mut it = range.split("-").map(|s| s.parse().unwrap());
            (it.next().unwrap(), it.next().unwrap())
        };
        let mut inv_sum_set: HashSet<i64> = HashSet::new();
        for i in 2..right.to_string().chars().count() + 1 {
            let mut lbound: i64 = 1;
            let mut pot_inv: i64 = lbound.to_string().repeat(i).parse().unwrap();
            while pot_inv <= right && lbound <= 9 {
                if pot_inv >= left && !inv_sum_set.contains(&pot_inv) {
                    invalid_sum += pot_inv;
                    inv_sum_set.insert(pot_inv);
                }
                lbound += 1;
                pot_inv = lbound.to_string().repeat(i).parse().unwrap();
            }
        }
        for i in 2..(right.to_string().chars().count()) + 1 {
            let mut lbound: i64 = {
                let ui = i;
                let left_length = left.to_string().chars().count();
                if left_length % ui == 0 {
                    left.to_string()[..(left_length / ui)].parse().unwrap()
                } else {
                    10_i64.pow((left_length / ui) as u32)
                }
            };
            let mut pot_inv: i64 = lbound.to_string().repeat(i).parse().unwrap();
            while pot_inv <= right {
                if pot_inv >= left && !inv_sum_set.contains(&pot_inv) {
                    invalid_sum += pot_inv;
                    inv_sum_set.insert(pot_inv);
                    println!("{pot_inv} is between {left} and {right}");
                }
                lbound += 1;
                pot_inv = lbound.to_string().repeat(i).parse().unwrap();
            }
        }
    }
    invalid_sum
}
