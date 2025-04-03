use std::{
    cmp::{min, max},
    io::{self, Read},
};


struct Section {
    ranges: Vec<Range>,
}


impl Section {
    fn parse(text: &str) -> Section {
        let ranges: Vec<_> = text.split('\n').skip(1)
            .map(|line| Range::parse(line))
            .collect();

        Section {
            ranges,
        }
    }
}


struct Range {
    dest: u64,
    source: u64,
    len: u64,
}


impl Range {
    fn parse(text: &str) -> Range {
        let fields: Vec<u64> = text.split(' ')
            .map(|field| field.parse().unwrap())
            .collect();

        Range {
            dest: fields[0],
            source: fields[1],
            len: fields[2],
        }
    }
}


fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let sections: Vec<_> = input.trim().split("\n\n").collect();
    let colon_i = sections[0].find(':').unwrap() + 1;
    let seeds: Vec<u64> = sections[0][colon_i..]
        .split_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect();

    let sections: Vec<_> = sections.iter().skip(1)
        .map(|text| Section::parse(text))
        .collect();

    let res = solve1(&seeds, &sections);
    println!("{}", res);

    let res = solve2(&seeds, &sections);
    println!("{}", res);
}


fn solve1(seeds: &[u64], sections: &[Section]) -> u64 {
    let mut res = 2_000_000_000;
    for &seed in seeds {
        let mut current = seed;
        for section in sections {
            for range in &section.ranges {
                if current >= range.source && current < range.source + range.len {
                    current = current + range.dest - range.source;
                    break;
                }
            }
        }

        res = min(res, current);
    }

    res
}


fn solve2(seeds: &[u64], sections: &[Section]) -> u64 {
    let mut seeds = make_pairs(seeds);
    sort_seeds(&mut seeds);

    for section in sections {
        let mut old_seeds = seeds.to_vec();
        seeds.clear();
        while old_seeds.len() > 0 {
            let seed_range = old_seeds.pop().unwrap();

            // If we find a range, add new values to seeds and old_seeds
            // else add all to seeds
            let mut found = false;
            for range in &section.ranges {
                if seed_range[0] < range.source + range.len &&
                    seed_range[1] > range.source {
                    found = true;
                    let start = max(seed_range[0], range.source);
                    let end = min(seed_range[1], range.source + range.len);
                    seeds.push([
                        start + range.dest - range.source,
                        end + range.dest - range.source,
                    ]);
                    if seed_range[0] < range.source {
                        old_seeds.push([seed_range[0], range.source]);
                    }
                    if seed_range[1] > range.source + range.len {
                        old_seeds.push([range.source + range.len, seed_range[1]]);
                    }

                    break;
                }
            };

            if !found {
                seeds.push(seed_range);
            }
        }

        sort_seeds(&mut seeds);
    }

    seeds[0][0]
}


/// Take the input seed list and turn into into ranges [a,b)
fn make_pairs(list: &[u64]) -> Vec<[u64;2]> {
    let mut res = Vec::new();
    let mut acc = Vec::new();
    for item in list {
        acc.push(*item);
        if acc.len() == 2 {
            res.push([acc[0], acc[0] + acc[1]]);
            acc.clear();
        }
    }

    res
}


fn sort_seeds(seeds: &mut Vec<[u64; 2]>) {
    seeds.sort();
    let mut i = 0;
    while i + 1 < seeds.len() {
        if seeds[i+1][0] <= seeds[i][1] {
            let new_seed = [seeds[i][0], seeds[i+1][1]];
            seeds.remove(i);
            seeds.remove(i);
            seeds.insert(i, new_seed);
        } else {
            i += 1;
        }
    }
}
