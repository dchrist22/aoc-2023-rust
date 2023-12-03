advent_of_code::solution!(1);

fn first_digit(s: &str) -> Option<char> {
    let mut result: Option<char> = None;
    for c in s.chars() {
        if c.is_ascii_digit() {
            result = Some(c);
            break;
        }
    }

    result
}

fn last_digit(s: &str) -> Option<char> {
    let mut result: Option<char> = None;
    for c in s.chars().rev() {
        if c.is_ascii_digit() {
            result = Some(c);
            break;
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut res: u32 = 0;

    for line in lines {
        let mut a_str = String::from("");
        a_str.push(first_digit(line)?);
        a_str.push(last_digit(line)?);
        res += a_str.parse::<u32>().unwrap();
    }

    Some(res)
}

#[rustfmt::skip]
mod unformatted {

    pub const SPELLED_DIGITS : [&str; 18]= [
        "1","2","3","4","5","6","7","8","9",
        "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine"
    ];

    pub fn sd_to_d (s:&str) -> Option<u32> {
        match s {
            "one"   => Some(1), "1" => Some(1),
            "two"   => Some(2), "2" => Some(2),
            "three" => Some(3), "3" => Some(3),
            "four"  => Some(4), "4" => Some(4),
            "five"  => Some(5), "5" => Some(5),
            "six"   => Some(6), "6" => Some(6),
            "seven" => Some(7), "7" => Some(7),
            "eight" => Some(8), "8" => Some(8),
            "nine"  => Some(9), "9" => Some(9),
            _       => None

            // s       => {
            //     let oc = s.chars().next();
            //     match oc {
            //         Some(c) if c.is_ascii_digit() =>
            //         {
            //             Some(c.to_digit(10)?)
            //         }
            //         Some(_) => None,
            //         None    => None

            //     }
            // }
        }
    }
}

use unformatted::*;

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut ret = 0;

    for line in lines {
        let mut v: Vec<(u32, u32)> = Vec::new();

        for sd in SPELLED_DIGITS {
            let index = line.find(sd);

            if let Some(i) = index {
                let i_u32 = <u32>::try_from(i).unwrap();
                v.push((i_u32, sd_to_d(sd)?));
            }

            let matches = line.match_indices(sd);
            for mat in matches {
                let i_u32 = <u32>::try_from(mat.0).unwrap();
                v.push((i_u32, sd_to_d(sd)?));
            }
        }

        let min_t = v.iter().min_by_key(|d| &d.0).unwrap();
        let max_t = v.iter().max_by_key(|d| &d.0).unwrap();

        let mut s = String::from("");
        s = s + &min_t.1.to_string();
        s = s + &max_t.1.to_string();

        let n: u32 = s.parse().unwrap();

        // println!("{:?}", line);
        // println!("{:?}", v);
        // println!("min: {:?}", min_t.1);
        // println!("max: {:?}", max_t.1);
        // println!("n {}", n);

        ret += n;
    }

    // println!("ret {}", ret);
    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
