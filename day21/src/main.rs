use std::collections::HashMap;
use std::io::stdin;

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
struct State {
    /// Player whose turn it is
    player: usize,
    poses: [i32;2],
    scores: [i32;2],
}

fn main() {
    let poses = get_starting_positions();

    let res = solve1(poses);
    println!("{}", res);

    let res = solve2(poses);
    println!("{}", res);
}

fn solve1(poses: [i32;2]) -> i32 {
    let mut poses = poses;
    let mut scores = [0;2];
    let mut index = 0;
    let mut dice = 1;
    let mut roll_count = 0;
    loop {
        roll_count += 3;
        for _ in 0..3 {
            poses[index] += dice;
            dice += 1;
        }

        // 1-10 inclusive
        poses[index] = (poses[index] - 1) % 10 + 1;
        scores[index] += poses[index];
        if scores[index] >= 1000 {
            break;
        }

        index = 1 - index;
    }

    scores[1-index] * roll_count
}

fn solve2(poses: [i32;2]) -> i64 {
    let mut dp: HashMap<State,i64> = HashMap::new();
    dp.insert(State {
        player: 0,
        poses,
        scores: [0,0],
    }, 1);

    for total_score in 0..43 {
        for score0 in 0..total_score+1 {
            let scores = [score0, total_score - score0];
            if scores[0] >= 21 || scores[1] >= 21 {
                continue;
            }
            for player in 0..2 {
                for pos0 in 1..11 {
                    for pos1 in 1..11 {
                        let state = State {
                            player,
                            poses: [pos0,pos1],
                            scores,
                        };
                        if let Some(&count) = dp.get(&state) {
                            roll_dice(&mut dp, state.clone(), count, 3);
                        }
                    }
                }
            }
        }
    }

    let mut res = 0;
    for (state,count) in dp {
        if state.scores[0] >= 21 {
            res += count;
        }
    }

    res
}

fn roll_dice(dp: &mut HashMap<State,i64>, state: State, count: i64, dice: i32) {
    if dice == 0 {
        let mut state = state.clone();
        state.scores[state.player] += state.poses[state.player];
        state.player = 1 - state.player;
        *dp.entry(state.clone()).or_insert(0) += count;
        return;
    }

    for roll in 1..4 {
        let mut state = state.clone();
        state.poses[state.player] += roll;
        state.poses[state.player] = (state.poses[state.player] - 1) % 10 + 1;
        roll_dice(dp, state, count, dice-1);
    }
}

fn get_starting_positions() -> [i32;2] {
    let stdin = stdin();

    let mut res = [0;2];
    for i in 0..res.len() {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        res[i] = get_starting_position(input.trim());
    }

    res
}

fn get_starting_position(input: &str) -> i32 {
    let ch = input.as_bytes().last().unwrap();
    let res = ch - b'0';
    res as i32
}
