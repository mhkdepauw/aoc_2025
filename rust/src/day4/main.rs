use rust::read_lines;

fn main() {
    let dummy_input: Vec<String> = "..@@.@@@@.,@@@.@.@.@@,@@@@@.@.@@,@.@@@@..@.,@@.@@@@.@@,.@@@@@@@.@,.@.@.@.@@@,@.@@@.@@@@,.@@@@@@@@.,@.@.@@@.@."
        .split(",").map(|s|s.to_string()).collect();
    let real_input: Vec<String> = read_lines(4);
    println!("{}", part1(&real_input));
    println!("{}", part2(&real_input));
}

fn part1(megavec: &Vec<String>) -> i32 {
    countremove_accesibles(megavec.clone().iter().map(|s| {s.chars().collect()}).collect()).0
}

fn part2(megavec: &Vec<String>) -> i32 {
    let mut imegavec: Vec<Vec<char>> = megavec.clone().iter().map(|s| {s.chars().collect()}).collect();
    let mut total_accessible = 0;
    let mut last_remove = 1;
    while last_remove != 0 {
        (last_remove, imegavec) = countremove_accesibles(imegavec);
        total_accessible += last_remove;
    }
    total_accessible
}

fn countremove_accesibles(mut imegavec: Vec<Vec<char>>) -> (i32, Vec<Vec<char>>){
    let mut accessible = 0;
    for i in 0..imegavec.len() {
        let inotfirst = i > 0;
        let inotlast = i < imegavec.len()-1;
        for j in 0..imegavec[i].len() {
            if imegavec[i][j] == '@' {
                let jnotfirst = j > 0;
                let jnotlast = j < imegavec[i].len()-1;
                let mut count = 0;
                if inotfirst {
                    if imegavec[i-1][j] == '@' { count+=1 }
                    if jnotfirst && imegavec[i-1][j-1] == '@' { count+=1 }
                    if jnotlast && imegavec[i-1][j+1] == '@' { count+=1 }
                };
                if jnotfirst && imegavec[i][j-1] == '@' { count+=1 }
                if jnotlast && imegavec[i][j+1] =='@' { count+=1 }
                if inotlast {
                    if imegavec[i+1][j] == '@' { count+=1 }
                    if jnotfirst && imegavec[i+1][j-1] == '@' { count+=1 }
                    if jnotlast && imegavec[i+1][j+1] == '@' { count+=1 }
                }
                if count < 4 {
                    accessible+=1;
                    imegavec[i][j] = '.';
                }
            }
        }
    }
    (accessible,imegavec)
}