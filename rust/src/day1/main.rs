use rust::read_lines;

fn main() {
    let dummy_input: Vec<String> = "L68,L30,R48,L5,R60,L55,L1,L99,R14,L82"
        .split(',')
        .map(|s| s.to_string())
        .collect();
    let real_input: Vec<String> = read_lines(1);
    println!("{}", part1(&real_input));
    println!("{}", part2(&real_input));
}

fn part1(megavec: &Vec<String>) -> i32 {
    let mut zero_counter: i32 = 0;
    let ops: Vec<i32> = megavec
        .iter()
        .map(|s| {
            if s.starts_with('L') {
                -s[1..].parse::<i32>().unwrap()
            } else {
                s[1..].parse::<i32>().unwrap()
            }
        })
        .collect();
    ops.iter().fold(50, |acc, x| {
        if ((acc + x) % 100) == 0 {
            zero_counter += 1;
            (acc + x) % 100
        } else {
            (acc + x) % 100
        }
    });
    zero_counter
}

fn part2(megavec: &Vec<String>) -> i32 {
    let mut zero_counter: i32 = 0;
    let ops = megavec.iter().map(|s| {
        if s.starts_with('L') {
            -s[1..].parse::<i32>().unwrap()
        } else {
            s[1..].parse::<i32>().unwrap()
        }
    });
    ops.fold(50, |acc: i32, x: i32| {
        zero_counter += x.abs() / 100;
        let nx = x % 100;
        if nx.is_positive() && acc + nx >= 100 || nx.is_negative() && nx.abs() >= acc && acc != 0 {
            zero_counter += 1;
            (acc + nx).rem_euclid(100)
        } else {
            (acc + nx).rem_euclid(100)
        }
    });
    zero_counter
}
