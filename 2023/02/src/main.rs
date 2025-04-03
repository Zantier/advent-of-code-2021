use std::io;


const COLOR_COUNT: usize = 3;


#[derive(Debug)]
struct Game {
    id: u32,
    grabs: Vec<Grab>,
}


impl Game {
    fn parse(text: &str) -> Game {
        let colon_index = text.find(':').unwrap();
        let id: u32 = text[5..colon_index].parse().unwrap();
        let grabs: Vec<_> = text[colon_index + 2..].split("; ")
            .map(|text| Grab::parse(text))
            .collect();

        Game {
            id,
            grabs,
        }
    }
}


#[derive(Debug)]
struct Grab {
    cubes: [u32;COLOR_COUNT],
}


impl Grab {
    fn parse(text: &str) -> Grab {
        let mut cubes = [0;COLOR_COUNT];

        for part in text.split(", ") {
            let fields: Vec<_> = part.split(' ').collect();
            let count: u32 = fields[0].parse().unwrap();
            let index = color_to_index(fields[1]);
            cubes[index] = count;
        }

        Grab {
            cubes,
        }
    }
}


fn main() {
    let games: Vec<_> = io::stdin().lines()
        .map(|line| Game::parse(&line.unwrap()))
        .collect();

    let res = solve1(&games);
    println!("{}", res);

    let res = solve2(&games);
    println!("{}", res);
}


fn color_to_index(color: &str) -> usize {
    if color == "red" {
        return 0;
    }

    if color == "green" {
        return 1;
    }

    if color == "blue" {
        return 2;
    }

    return 0;
}


fn solve1(games: &[Game]) -> u32 {
    let mut res = 0;

    for game in games {
        let mut impossible = false;
        for grab in &game.grabs {
            impossible = impossible || grab.cubes[0] > 12 || grab.cubes[1] > 13 || grab.cubes[2] > 14;
        }

        if !impossible {
            res += game.id;
        }
    }

    return res;
}


fn solve2(games: &[Game]) -> u32 {
    let mut res = 0;

    for game in games {
        let mut fewest = [0; COLOR_COUNT];
        for grab in &game.grabs {
            for i in 0..COLOR_COUNT {
                fewest[i] = std::cmp::max(fewest[i], grab.cubes[i]);
            }
        }

        res += fewest.iter().product::<u32>();
    }

    return res;
}
