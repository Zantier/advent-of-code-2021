use std::io::stdin;

fn main() {
    let stdin = stdin();
    let mut algo = String::new();
    stdin.read_line(&mut algo).unwrap();

    let algo = algo.trim();

    // Blank line
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let mut image = Vec::new();
    loop {
        let mut input = String::new();
        if stdin.read_line(&mut input).unwrap() == 0 {
            break;
        }

        image.push(input.trim().as_bytes().to_vec());

    }

    let res = solve1(algo.as_bytes(), &image);
    println!("{}", res);
}

fn solve1(algo: &[u8], image: &[Vec<u8>]) -> i32 {
    let mut image = extend_image(image, 10);
    let height = image.len();
    let width = image[0].len();

    for _ in 0..2 {
        let old_image = image.clone();
        for i in 1..height-1 {
            for j in 1..width-1 {
                let mut algo_index = 0;
                for i2 in i-1..i+2 {
                    for j2 in j-1..j+2 {
                        let bit = if old_image[i2][j2] == b'#' { 1 } else { 0 };
                        algo_index = 2*algo_index + bit;
                    }
                }

                image[i][j] = algo[algo_index];
            }
        }
    }

    count_light(&image)
}

fn extend_image(input: &[Vec<u8>], pad: usize) -> Vec<Vec<u8>> {
    let width = input[0].len() + 2*pad;
    let mut res: Vec<Vec<u8>> = Vec::new();
    for _ in 0..pad+1 {
        res.push(vec![b'.';width]);
    }

    for i in 0..input.len() {
        let mut row = vec![b'.'; pad];
        row.extend(&input[i]);
        row.extend(vec![b'.'; pad]);
        res.push(row);
    }

    for _ in 0..pad+1 {
        res.push(vec![b'.';width]);
    }

    res
}

fn count_light(image: &[Vec<u8>]) -> i32 {
    let mut res = 0;
    let bad = 2;
    for row in image.iter().take(image.len()-bad).skip(bad) {
        for ch in row.iter().take(row.len()-bad).skip(bad) {
            if *ch == b'#' {
                res += 1;
            }
        }
    }

    res
}

fn _print_image(image: &[Vec<u8>]) {
    println!("");
    for row in image.iter() {
        println!("{}", String::from_utf8(row.clone()).unwrap());
    }
}
