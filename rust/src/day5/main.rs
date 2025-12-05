use rust::read_lines;
use std::fs::read_to_string;

fn main() {
    let dummy_input: Vec<String> = [
        "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
    ].iter().map(|s| s.to_string()).collect();
    let real_input: Vec<String> = read_lines(5);
    println!("{}", part1(&real_input));
    println!("{}", part2(&dummy_input));
}

fn part1(megavec: &Vec<String>) -> i64 {
    let mut spoil_count = 0;
    let (rangevec, checkvec): (Vec<(i64, i64)>, Vec<i64>) = splitvec(megavec);
    println!("{:?}\n{:?}",rangevec,checkvec);
    for id in checkvec {
        if rangevec.iter().any(|(l,r)|  id >= *l && id <= *r ) {
            spoil_count += 1;
        }
    }
    spoil_count
}

fn part2(megavec: &Vec<String>) -> i32 {
    0
}

fn splitvec(megavec: &Vec<String>) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut split = megavec.split(|s| s == "");
    (
        split
            .next()
            .unwrap()
            .iter()
            .map(|s1| {
                let mut it = s1.split('-').map(|s2| {
                    s2.parse::<i64>().unwrap()
                });
                (it.next().unwrap(), it.next().unwrap())
            })
            .collect::<Vec<(i64, i64)>>()
            .to_vec(),
        split
            .next()
            .unwrap()
            .iter()
            .map(|s| s.parse().unwrap())
            .collect(),
    )
}
