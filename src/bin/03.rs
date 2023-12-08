advent_of_code::solution!(3);

#[derive(Debug, PartialEq)]
enum Tile {
    Digit(char),
    Symbol(char),
    Dot,
}
use crate::Tile::*;

#[derive(Debug)]
struct Schematic(Vec<Vec<Tile>>);

impl Schematic {
    fn get_tile(&self, x: isize, y: isize) -> Option<&Tile> {
        let mut ret: Option<&Tile> = None;

        if x >= 0 && y >= 0 {
            let xu: usize = usize::try_from(x).ok()?;
            let yu: usize = usize::try_from(y).ok()?;

            let row = self.0.get(xu);
            if let Some(v) = row {
                ret = v.get(yu);
            }
        }

        ret
    }

    fn is_adj_to_sym(&self, x: isize, y: isize) -> bool {
        let nbs = [
            self.get_tile(x - 1, y),     // n
            self.get_tile(x - 1, y + 1), // no
            self.get_tile(x, y + 1),     // o
            self.get_tile(x + 1, y + 1), // so
            self.get_tile(x + 1, y),     // s
            self.get_tile(x + 1, y - 1), // sw
            self.get_tile(x, y - 1),     // w
            self.get_tile(x - 1, y - 1), // nw
        ];

        // let nbsf = nbs.into_iter().flatten();

        for nb in nbs {
            if let Some(&Symbol(_)) = nb {
                return true;
            }
        }
        false
    }

    fn _size(&self) -> (usize, usize) {
        let snd = match self.0.get(0) {
            Some(v) => v.len(),
            None => 0,
        };

        (self.0.len(), snd)
    }

    fn get_numbers(&self) -> Vec<(Vec<(usize, usize)>, usize)> {
        let rows = &self.0;

        let mut numbers: Vec<(Vec<(usize, usize)>, usize)> = Vec::new();

        for (i, row) in rows.iter().enumerate() {
            let mut enum_tiles = row.iter().enumerate();

            while enum_tiles.len() > 0 {
                let mut s = String::from("");
                let mut is: Vec<(usize, usize)> = Vec::new();
                loop {
                    let next = enum_tiles.next();
                    match next {
                        Some((j, &Digit(d))) => {
                            s.push(d);
                            is.push((i, j));
                        }
                        _ => {
                            break;
                        }
                    }
                }

                if !is.is_empty() {
                    let n: usize = s.parse::<usize>().unwrap();
                    numbers.push((is, n));
                }
            }
        }

        numbers
    }
}

fn char_to_tile(c: char) -> Tile {
    match c {
        '.' => Dot,
        c if c.is_ascii_digit() => Digit(c),
        // Digit(u8::try_from(c.to_digit(10).unwrap()).unwrap()),
        c => Symbol(c),
    }
}

fn parse_schematic(s: &str) -> Schematic {
    let lines = s.lines();

    let mut v: Vec<Vec<Tile>> = Vec::new();

    for line in lines {
        let mut vl: Vec<Tile> = vec![];
        for c in line.chars() {
            vl.push(char_to_tile(c));
        }

        v.push(vl);
    }

    Schematic(v)
}

pub fn part_one(input: &str) -> Option<u32> {
    let s: Schematic = parse_schematic(input);
    // for line in input.lines() {
    //     println!("{:}", line);
    // }

    // println!("{:?}", s.get_numbers());

    // let (len_x, len_y) = s.size();
    // for i in 0..len_x {
    //     for j in 0..len_y {
    //         let ii = isize::try_from(i).unwrap();
    //         let ij = isize::try_from(j).unwrap();
    //         println!("i{1}, j{2}: {0}", s.is_adj_to_sym(ii,ij), ii, ij);
    //     }
    // }

    let mut res = 0;
    for (is, n) in s.get_numbers() {
        'inner: for i in is {
            let (x, y) = i;
            let xi: isize = isize::try_from(x).ok()?;
            let yi: isize = isize::try_from(y).ok()?;
            if s.is_adj_to_sym(xi, yi) {
                res += n;
                break 'inner;
            }
        }
    }

    let res_u32: Option<u32> = u32::try_from(res).ok();
    res_u32
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
