use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|line| line.split(":").last().unwrap().trim())
        .collect::<Vec<&str>>();

    let result = input
        .iter()
        .map(|x| x.split("|").map(|x| x.trim()).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut points = 0;

    for (i, r) in result.iter().enumerate() {
        println!("Game {}: {:?}", i + 1, r);
        let winning_numbers = r
            .first()
            .unwrap()
            .split(" ")
            .map(|x| x.parse::<u32>())
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect::<Vec<u32>>();
        let numbers = r
            .last()
            .unwrap()
            .split(" ")
            .map(|x| x.parse::<u32>())
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap())
            .collect::<Vec<u32>>();

        let numbers_set: HashSet<_> = HashSet::from_iter(numbers.iter());
        let winning_numbers_set: HashSet<_> = HashSet::from_iter(winning_numbers.iter());
        let valid_winning_numbers = numbers_set.intersection(&winning_numbers_set).count();

        println!(
            "Winning numbers: {:?}\nNumbers: {:?}\nSet intersect: {:?}",
            winning_numbers, numbers, valid_winning_numbers
        );

        if valid_winning_numbers == 0 {
            continue;
        } else {
            println!(
                "{}",
                (1..=valid_winning_numbers).reduce(|a, b| a * 2).unwrap()
            );
            points += (1..=valid_winning_numbers).reduce(|a, b| a * 2).unwrap();
        }
    }

    println!("{}", points);
}
