use std::collections::HashMap;

fn main() {
    day1();
    day1pt2();
}

fn day1() {
    let input = include_str!("day1.txt");
    let result: u32 = input
        .lines()
        .map(|line| {
            let first = line
                .chars()
                .filter(|x| x.is_numeric())
                .take(1)
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap();

            let last = line
                .chars()
                .filter(|x| x.is_numeric())
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap();

            return (first * 10) + last;
        })
        .sum();
    println!("{:?}", result);
}

fn day1pt2() {
    let input = include_str!("day1.txt");
    let mut numbers: HashMap<String, String> = HashMap::new();

    numbers.insert(String::from("one"), String::from("1"));
    numbers.insert(String::from("two"), String::from("2"));
    numbers.insert(String::from("three"), String::from("3"));
    numbers.insert(String::from("four"), String::from("4"));
    numbers.insert(String::from("five"), String::from("5"));
    numbers.insert(String::from("six"), String::from("6"));
    numbers.insert(String::from("seven"), String::from("7"));
    numbers.insert(String::from("eight"), String::from("8"));
    numbers.insert(String::from("nine"), String::from("9"));

    let result: Vec<String> = input
        .lines()
        .map(|line| {
            let mut replaced_line = line.to_owned();

            for ele in &numbers {
                replaced_line = replaced_line.replace(&(*ele.0), &((*ele.0).clone() + &(*ele.1) + &(*ele.0).clone()));
            }

            return replaced_line;
        })
        .collect();

    let result: u32 = result
        .iter()
        .map(|line| {
            let first = line
                .chars()
                .filter(|x| x.is_numeric())
                .take(1)
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap();

            let last = line
                .chars()
                .filter(|x| x.is_numeric())
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap();

            return (first * 10) + last;
        })
        .sum();
    println!("{:?}", result);
}
