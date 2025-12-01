use rust::read_lines;

fn main() {
    let megavec = read_lines(0);
    println!("{}", part1(megavec));
}

fn part1(megavec: Vec<String>) -> i32 {
    let (mut leftvec, mut rightvec): (Vec<i32>, Vec<i32>) = megavec
        .iter()
        .map(|s| {
            let mut sp = s.splitn(2, ' ').map(|s2| s2.trim().parse::<i32>().unwrap());
            (sp.next().unwrap(), sp.next().unwrap())
        })
        .unzip();
    leftvec.sort();
    rightvec.sort();
    leftvec
        .iter()
        .zip(rightvec)
        .map(|(l, r)| (l - r).abs())
        .sum()
}
