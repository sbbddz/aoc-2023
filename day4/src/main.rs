use std::collections::HashSet;

#[derive(Debug)]
struct Card<'a> {
    game: &'a Vec<&'static str>,
    copies: u32,
}

fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|line| line.split(":").last().unwrap().trim())
        .collect::<Vec<&str>>();

    let result = input
        .iter()
        .map(|x| x.split("|").map(|x| x.trim()).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut cards = result
        .iter()
        .map(|x| Card { game: x, copies: 0 })
        .collect::<Vec<Card>>();

    let mut points = 0;

    // PART-1
    for r in result.iter() {
        let valid_winning_numbers = get_winning_numbers(r);

        if valid_winning_numbers == 0 {
            continue;
        } else {
            points += (1..=valid_winning_numbers).reduce(|a, _| a * 2).unwrap();
        }
    }

    // PART-2 ( taking too long, not fast )
    for (i, r) in result.iter().enumerate() {
        for _ in 0..cards.get_mut(i).unwrap().copies + 1 {
            let valid_winning_numbers = get_winning_numbers(r);
            for j in 1..=valid_winning_numbers {
                cards.get_mut(i + j).unwrap().copies += 1;
            }
        }
    }

    println!("{}", points);
    println!("{}", cards.iter().map(|x| x.copies + 1).sum::<u32>());
    for (i, c) in cards.iter().enumerate() {
        println!("Card: {} - {:?}", i + 1, c);
    }
}

fn get_winning_numbers(r: &Vec<&str>) -> usize {
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
    return valid_winning_numbers;
}
