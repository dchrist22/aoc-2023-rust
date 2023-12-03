advent_of_code::solution!(2);

#[derive(Debug)]
enum Color {
    Blue,
    Red,
    Green,
}

fn str_to_color(s: &str) -> Option<Color> {
    match s {
        "blue" => Some(Color::Blue),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        _ => None,
    }
}

fn to_cube_tuple(s: &str) -> (u32, Color) {
    let v: Vec<&str> = s.split_ascii_whitespace().collect();

    let n: u32 = v[0].parse::<u32>().unwrap();

    let c: Option<Color> = str_to_color(v[1]);

    (n, c.unwrap())
}

fn parse_game(s: &str) -> Vec<Vec<(u32, Color)>> {
    let v: Vec<&str> = s.split_terminator(':').collect();

    let subsets: Vec<&str> = v[1].split_terminator(';').collect();

    // subsets.into_iter().map(|ss| {ss.split_terminator(',')});

    let mut subsets_split: Vec<Vec<&str>> = Vec::new();

    for subset in subsets {
        subsets_split.push(subset.split_terminator(',').collect());
    }

    let mut game: Vec<Vec<(u32, Color)>> = Vec::new();

    for subset in subsets_split {
        let mut v: Vec<(u32, Color)> = Vec::new();
        for item in subset {
            v.push(to_cube_tuple(item));
        }
        game.push(v);
    }

    game
}

fn is_possible(game: &Vec<Vec<(u32, Color)>>) -> bool {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut res = true;

    'outer: for subset in game {
        for item in subset {
            match item {
                (n, Color::Red) if n > &max_red => {
                    res = false;
                    break 'outer;
                }

                (n, Color::Green) if n > &max_green => {
                    res = false;
                    break 'outer;
                }

                (n, Color::Blue) if n > &max_blue => {
                    res = false;
                    break 'outer;
                }

                (_, _) => true,
            };
        }
    }
    res
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut result: u32 = 0;

    for (i, line) in lines.enumerate() {
        let i_u32 = u32::try_from(i).unwrap() + 1;

        let game = parse_game(line);

        if is_possible(&game) {
            result += i_u32;
        }

        // println!("i_u32 {}", i_u32);
        // println!("{}", is_possible(&game));
        // println!("{:?}", game);
        // println!("");
    }

    // println!("{}", result);
    Some(result)
}

fn minimum_cubes(game: &Vec<Vec<(u32, Color)>>) -> (u32, u32, u32) {
    let mut min_r = 1;
    let mut min_g = 1;
    let mut min_b = 1;

    for subset in game {
        for cubes in subset {
            match cubes {
                (n, Color::Red) if n > &min_r => min_r = *n,
                (n, Color::Green) if n > &min_g => min_g = *n,
                (n, Color::Blue) if n > &min_b => min_b = *n,
                (_, _) => {}
            }
        }
    }

    (min_r, min_g, min_b)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut res: u32 = 0;

    for line in lines {
        let game = parse_game(line);
        let (r, g, b) = minimum_cubes(&game);

        // println!("{:?}", game);
        // println!("r {} g {} b {}", r,g,b);

        res += r * g * b;
    }
    // println!("{}", res);
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
