use std::io::stdin;

fn main() {
    let stdin = stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let bits: Vec<_> = input.trim().chars()
        .flat_map(|ch| nibble_to_bits(ch.to_digit(16).unwrap() as u8))
        .collect();

    let res = solve1(&bits);
    println!("{}", res);

    let res = solve2(&bits);
    println!("{}", res);
}

fn solve1(bits: &[u8]) -> i32 {
    helper1(bits, &mut 0)
}

fn solve2(bits: &[u8]) -> i64 {
    helper2(bits, &mut 0)
}

fn nibble_to_bits(nibble: u8) -> Vec<u8> {
    let mut res = Vec::new();
    for i in (0..4).rev() {
        let bit = if nibble & (1<<i) > 0 { 1 } else { 0 };
        res.push(bit);
    }

    res
}

fn helper1(bits: &[u8], index: &mut usize) -> i32 {
    let mut res = 0;
    let version = parse_bits(bits, 3, index);
    res += version;
    let type_id = parse_bits(bits, 3, index);

    match type_id {
        4 => {
            // Literal value

            let mut val: i64 = 0;
            loop {
                let bit = parse_bits(bits, 1, index);
                let part = parse_bits(bits, 4, index);
                val = 16*val + part as i64;

                if bit == 0 {
                    break;
                }
            }
        },
        _ => {
            // Other operator

            let length_type_id = parse_bits(bits, 1, index);
            match length_type_id {
                0 => {
                    let bit_count = parse_bits(bits, 15, index);
                    let start_index = *index;
                    loop {
                        let val = helper1(bits, index);
                        res += val;
                        if *index >= start_index + bit_count as usize {
                            break;
                        }
                    }

                },
                _ => {
                    let packet_count = parse_bits(bits, 11, index);
                    for _ in 0..packet_count {
                        let val = helper1(bits, index);
                        res += val;
                    }
                },
            }
        },
    }

    res
}

fn helper2(bits: &[u8], index: &mut usize) -> i64 {
    let _version = parse_bits(bits, 3, index);
    let type_id = parse_bits(bits, 3, index);

    if type_id == 4 {
        // Literal value

        let mut val: i64 = 0;
        loop {
            let bit = parse_bits(bits, 1, index);
            let part = parse_bits(bits, 4, index);
            val = 16*val + part as i64;

            if bit == 0 {
                break val;
            }
        }
    } else {
        // Other operator

        let values = parse_packet_list(bits, index);
        match type_id {
            0 => values.iter().sum(),
            1 => values.iter().product(),
            2 => *values.iter().min().unwrap(),
            3 => *values.iter().max().unwrap(),
            5 => if values[0] > values[1] { 1 } else { 0 },
            6 => if values[0] < values[1] { 1 } else { 0 },
            7 => if values[0] == values[1] { 1 } else { 0 },
            _ => panic!("oops"),
        }
    }
}

fn parse_packet_list(bits: &[u8], index: &mut usize) -> Vec<i64> {
    let mut res = Vec::new();
    let length_type_id = parse_bits(bits, 1, index);
    match length_type_id {
        0 => {
            let bit_count = parse_bits(bits, 15, index);
            let start_index = *index;
            loop {
                let val = helper2(bits, index);
                res.push(val);
                if *index >= start_index + bit_count as usize {
                    break;
                }
            }

        },
        _ => {
            let packet_count = parse_bits(bits, 11, index);
            for _ in 0..packet_count {
                let val = helper2(bits, index);
                res.push(val);
            }
        },
    }

    res
}

fn parse_bits(bits: &[u8], bit_count: usize, index: &mut usize) -> i32 {
    let mut res = 0;
    for _ in 0..bit_count {
        res *= 2;
        res += bits[*index] as i32;
        *index += 1;
    }

    res
}
