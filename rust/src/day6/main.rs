use std::fs;
use std::fs::read_to_string;
use std::str::Chars;
use rust::read_lines;

fn main() {
    let dummy_input: Vec<String> = read_to_string("src/day6/dummy_input.txt")
        .expect("Failed to get file as string")
        .lines()
        .map(|l| l.to_string())
        .collect();
    let real_input: Vec<String> = read_lines(6);
    println!("{}", part1(&real_input));
    println!("{}", part2(&real_input));
}

fn part1(megavec: &Vec<String>) -> i64 {
    let mut total_count: i64 = 0;
    let (ops, num_list): (Vec<String>,Vec<Vec<i64>>) = to_op_list(megavec);
    for i in 0..num_list.len() {
        if ops[i] == "*" { total_count += num_list[i].iter().product::<i64>()}
        else { total_count += num_list[i].iter().sum::<i64>() }
    }
    total_count
}

fn part2(megavec: &Vec<String>) -> i64 {
    let mut total_count: i64 = 0;
    let megavec_chars: Vec<Vec<char>> = megavec.iter().map(|s|s.chars().collect()).collect();
    let op_char_list: &Vec<char> = megavec_chars.last().unwrap();
    let mut num_start_list: Vec<usize> = vec![];
    for i in 0..megavec.last().unwrap().chars().count() {
        if op_char_list[i] != ' ' {
            num_start_list.push(i);
        }
    }
    let mut full_numbers_list_chars: Vec<Vec<Vec<char>>> = vec![];
    for i in 0..num_start_list.len() {
        let mut numbers_list_chars: Vec<Vec<char>> = vec![];
        for s in &megavec_chars[..megavec_chars.len()-1] {
            if i < num_start_list.len()-1 {
                numbers_list_chars.push(s[num_start_list[i]..num_start_list[i+1]-1].to_vec());
            }
            else {
                numbers_list_chars.push(s[num_start_list[i]..].to_vec());
            }
        }
        full_numbers_list_chars.push(numbers_list_chars);
    }
    // println!("{:?}",full_numbers_list_chars);
    let mut int_listlist: Vec<Vec<i64>> = vec![];
    for list in full_numbers_list_chars {
        let mut int_list: Vec<i64> = vec![];
        for j in 0..list[0].len() {
            let mut to_parse: Vec<char> = vec![];
            for i in 0..list.len() {
                to_parse.push(list[i][j]);
            }
            int_list.push(to_parse.iter().filter_map(|c| if c.is_digit(10) { Some(c) } else { None }).collect::<String>().parse().unwrap());
        }
        int_listlist.push(int_list);
    }
    let op_list: Vec<&char> = op_char_list.iter().filter(|&&c| c == '*' || c == '+').collect();
    for i in 0..op_list.len() {
        if *op_list[i] == '*' {
            total_count += int_listlist[i].iter().product::<i64>();
        }
        else {
            total_count += int_listlist[i].iter().sum::<i64>();
        }
    }
    // println!("{:?}",int_listlist);
    total_count
}

fn to_op_list(megavec: &Vec<String>) -> (Vec<String>, Vec<Vec<i64>>) {
    let processed: Vec<Vec<String>> = megavec
        .iter()
        .map(|s| {
            s.split(" ")
                .filter(|s1| !s1.is_empty())
                .map(|s2| s2.to_string())
                .collect::<Vec<String>>()
        })
        .collect();
    let mut transposed_m: Vec<Vec<i64>> = vec![];
    for i in 0..processed[0].len() {
        let mut transposed: Vec<i64> = vec![];
        for j in 0..processed.len() - 1 {
            transposed.push(processed[j][i].parse().unwrap());
        }
        transposed_m.push(transposed);
    }
    (processed[processed.len() - 1].clone(), transposed_m)
}
