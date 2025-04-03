use std::io;

#[derive(Debug)]
struct Race {
    time: u64,
    dist: u64,
}


impl Race {
    fn parse_multiple(lines: &[String]) -> Vec<Race> {
        let times = Self::parse_line_multiple(&lines[0]);
        let dists = Self::parse_line_multiple(&lines[1]);
        times.into_iter().zip(dists)
            .map(|(time, dist)| Race {time, dist})
            .collect()
    }


    fn parse_single(lines: &[String]) -> Race {
        let time = Self::parse_line_single(&lines[0]);
        let dist = Self::parse_line_single(&lines[1]);
        Race {time, dist}
    }


    fn parse_line_multiple(line: &str) -> Vec<u64> {
        line.split_whitespace()
            .skip(1)
            .map(|x| x.parse().unwrap())
            .collect()
    }


    fn parse_line_single(line: &str) -> u64 {
        let joined: String = line.split_whitespace()
            .skip(1)
            .collect();
        joined.parse().unwrap()
    }


    fn get_wins(&self) -> u64 {
        // Get the velocities that cover the exact distance
        if self.time * self.time < 4 * self.dist {
            return 0;
        }

        let discriminant = self.time * self.time - 4 * self.dist;
        let discriminant = (discriminant as f64 / 4.0).sqrt();
        let low = (self.time as f64 / 2.0 - discriminant).floor() as u64;
        let high = (self.time as f64 / 2.0 + discriminant).ceil() as u64;
        high - low - 1
    }


    #[allow(dead_code)]
    fn get_wins_slow(&self) -> u64 {
        let mut wins = 0;
        for speed in 1..self.time-1 {
            let time = self.time - speed;
            let dist = speed * time;
            if dist > self.dist {
                wins += 1;
            }
        }

        wins
    }
}


fn main() {
    let lines: Vec<_> = io::stdin().lines()
        .map(|line| line.unwrap())
        .collect();

    let races = Race::parse_multiple(&lines);
    let res = solve1(&races);
    println!("{}", res);

    let race = Race::parse_single(&lines);
    let res = solve2(race);
    println!("{}", res);
}


fn solve1(races: &[Race]) -> u64 {
    let mut res = 1;

    for race in races {
        let wins = race.get_wins();
        res *= wins;
    }

    res
}


fn solve2(race: Race) -> u64 {
    race.get_wins()
}
