fn main() {
    day1();
    day1pt2();
}

fn day1() {
    let input = include_str!("day1.txt");
    let result: u32 = input
        .lines()
        .map(|line| {
            return (line
                .chars()
                .find(|x| x.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap()
                * 10)
                + line
                    .chars()
                    .rev()
                    .find(|x| x.is_ascii_digit())
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
        })
        .sum();
    println!("DAY 1 - PT1: {:?}", result);
}

fn day1pt2() {
    let input = include_str!("day1.txt");
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];

    let result: u32 = input
        .lines()
        .map(|line| {
            let mut matches: Vec<(usize, &str)> = numbers
                .iter()
                .map(|&x| line.match_indices(x).collect())
                .filter(|x| !Vec::is_empty(x))
                .flatten()
                .collect();
            matches.sort_by(|x, y| x.0.cmp(&y.0));

            let first_result = if matches.first().unwrap().1.len() == 1 {
                matches.first().unwrap().1.parse::<u32>().unwrap()
            } else {
                (numbers
                    .iter()
                    .position(|&x| x == matches.first().unwrap().1)
                    .unwrap()
                    + 1)
                .try_into()
                .unwrap()
            };

            let last_result = if matches.last().unwrap().1.len() == 1 {
                matches.last().unwrap().1.parse::<u32>().unwrap()
            } else {
                (numbers
                    .iter()
                    .position(|&x| x == matches.last().unwrap().1)
                    .unwrap()
                    + 1)
                .try_into()
                .unwrap()
            };

            return first_result * 10 + last_result;
        })
        .sum();

    println!("DAY 1 - PT2: {:?}", result);
}
