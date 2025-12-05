use rust::read_lines;

fn main() {
    let dummy_input: Vec<String> = [
        "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let real_input: Vec<String> = read_lines(5);
    println!("{}", part1(&real_input));
    println!("{}", part2(&real_input));
}

fn part1(megavec: &Vec<String>) -> i64 {
    let mut spoil_count = 0;
    let (rangevec, checkvec): (Vec<(i64, i64)>, Vec<i64>) = splitvec(megavec);
    for id in checkvec {
        if rangevec.iter().any(|(l, r)| id >= *l && id <= *r) {
            spoil_count += 1;
        }
    }
    spoil_count
}

fn part2(megavec: &Vec<String>) -> i64 {
    let mut fresh_ids = 0;
    let mut merged_ranges: Vec<(i64, i64)> = vec![];
    let (mut rangevec, _): (Vec<(i64, i64)>, _) = splitvec(megavec);
    for range in &rangevec {
        let mut obsolete_ranges: Vec<(i64, i64)> = vec![];
        let &(mut ml, mut mr) = range;
        for (x, y) in merged_ranges.clone() {
            let x_in_mrange = ml <= x && x <= mr;
            let y_in_mrange = ml <= y && y <= mr;
            let xy_contains_mrange = x < ml && mr < y;
            let mrange_contains_xy = x_in_mrange && y_in_mrange;
            if x_in_mrange {
                if !mrange_contains_xy {
                    mr = y;
                }
                obsolete_ranges.push((x, y));
            } else if y_in_mrange {
                ml = x;
                obsolete_ranges.push((x, y));
            } else if xy_contains_mrange {
                ml = x;
                mr = y;
                obsolete_ranges.push((x, y));
            }
        }
        delete_ranges(obsolete_ranges, &mut merged_ranges);
        merged_ranges.push((ml, mr));
    }
    for (l, r) in merged_ranges {
        fresh_ids += (r - l) + 1;
    }
    fresh_ids
}

fn splitvec(megavec: &Vec<String>) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut split = megavec.split(|s| s == "");
    (
        split
            .next()
            .unwrap()
            .iter()
            .map(|s1| {
                let mut it = s1.split('-').map(|s2| s2.parse::<i64>().unwrap());
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

fn delete_ranges(ranges: Vec<(i64, i64)>, list: &mut Vec<(i64, i64)>) {
    for range in ranges {
        list.remove(list.iter().position(|(x, y)| (*x, *y) == (range)).unwrap());
    }
}
