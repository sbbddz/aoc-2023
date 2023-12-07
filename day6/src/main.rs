fn main() {
    let input_part1 = include_str!("input.txt")
        .lines()
        .map(|x| {
            x.split(":")
                .last()
                .unwrap()
                .trim()
                .split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let input_part2 = include_str!("input.txt")
        .lines()
        .map(|x| {
            vec![x.split(":")
                .last()
                .unwrap()
                .trim()
                .split(" ")
                .filter(|x| !x.is_empty())
                .collect::<Vec<&str>>()
                .join("")
                .parse::<u64>()
                .unwrap()]
        })
        .collect::<Vec<Vec<u64>>>();
    calculate_valid_ways_to_win_from_input(&input_part1);
    calculate_valid_ways_to_win_from_input(&input_part2);
}

fn calculate_valid_ways_to_win_from_input(input: &Vec<Vec<u64>>) {
    let mut results = vec![];
    for i in 0..input.get(0).unwrap().len() {
        let time = input.get(0).unwrap().get(i).unwrap();
        let distance = input.get(1).unwrap().get(i).unwrap();
        let mut valid_ways_to_win = 0;

        for j in 0..*time {
            let remaining_time = time - j;
            let travel_distance = remaining_time * j;

            if travel_distance <= *distance {
            } else {
                valid_ways_to_win += 1;
            }
        }

        results.push(valid_ways_to_win);
        println!("Time: {time}, Distance: {distance}, Valid ways to win: {valid_ways_to_win}");
    }

    println!("{}", results.iter().product::<u64>());
}
