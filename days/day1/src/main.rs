use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, BufRead},
};

fn p1(mut left: Vec<u32>, mut right: Vec<u32>) -> u32 {
    left.sort();
    right.sort();

    let sum: u32 = left
        .into_iter()
        .zip(right.into_iter())
        .fold(0, |sum, (curr_l, curr_r)| sum + curr_r.abs_diff(curr_l));
    sum
}

fn p2(mut left: Vec<u32>, right: Vec<u32>) -> u32 {
    let r_buckets: HashMap<u32, u32> =
        right
            .into_iter()
            .fold(Default::default(), |mut buckets, elem| {
                let v = buckets.get_mut(&elem);
                if let Some(v) = v {
                    *v += 1;
                } else {
                    buckets.insert(elem, 1);
                }
                buckets
            });

    left.sort();

    left.into_iter().fold(0, |sum, elem| {
        sum + elem * r_buckets.get(&elem).unwrap_or(&0)
    })
}

fn main() {
    let file = File::open("input").unwrap();
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let (l, r) = line.split_once("   ").unwrap();
        left.push(l.parse().unwrap());
        right.push(r.parse().unwrap());
    }

    let mut args = env::args();
    args.next();
    let err_str =
        "Expected either part1 or part2 to be specified as the first command line argument.";
    let Some(p1_or_2) = args.next() else {
        panic!("{err_str}")
    };
    let p1_or_2 = p1_or_2.split_whitespace().next().unwrap();
    let sol = if "part1" == p1_or_2 {
        p1(left, right)
    } else if "part2" == p1_or_2 {
        p2(left, right)
    } else {
        panic!("{err_str}");
    };

    println!("{p1_or_2} solution: {sol}")
}
