fn main() {
    day2();
}

#[derive(Debug)]
struct Game {
    id: u32,
    // RED GREEN BLUE
    set: (u32, u32, u32),
    valid: bool,
}

fn day2() {
    let input: Vec<Game> = include_str!("day2.txt")
        .lines()
        .map(|line| {
            let idx = line.find(":").unwrap();
            let game_id = &line[5..idx];
            let sets_raw = &line[idx + 2..line.len()];
            let sets: Vec<Vec<&str>> = sets_raw
                .split(";")
                .map(|x| x.trim().split(",").map(|x| x.trim()).collect::<Vec<&str>>())
                .collect();

            // let parsed_sets:  = vec![];
            let mut break_loop = false;
            let mut game_valid = true;

            for set in &sets {
                if break_loop == true {
                    break;
                }

                let a = set
                    .iter()
                    .map(|&x| x.split_whitespace().collect::<Vec<_>>())
                    .collect::<Vec<_>>();

                for b in &a {
                    if !check_valid_set(b) {
                        game_valid = false;
                        break_loop = true;
                        break;
                    }
                }
            }

            let max_red = sets_raw
                .split(&[';', ','][..])
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.trim())
                .collect::<Vec<&str>>()
                .iter()
                .filter(|x| x.contains("red"))
                .map(|&x| x.chars().filter(|x| x.is_ascii_digit()).collect::<String>())
                .map(|x| x.parse::<u32>().unwrap())
                .max()
                .unwrap();

            let max_blue = sets_raw
                .split(&[';', ','][..])
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.trim())
                .collect::<Vec<&str>>()
                .iter()
                .filter(|x| x.contains("blue"))
                .map(|&x| x.chars().filter(|x| x.is_ascii_digit()).collect::<String>())
                .map(|x| x.parse::<u32>().unwrap())
                .max()
                .unwrap();

            let max_green = sets_raw
                .split(&[';', ','][..])
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.trim())
                .collect::<Vec<&str>>()
                .iter()
                .filter(|x| x.contains("green"))
                .map(|&x| x.chars().filter(|x| x.is_ascii_digit()).collect::<String>())
                .map(|x| x.parse::<u32>().unwrap())
                .max()
                .unwrap();

            println!("{:?}", (max_red, max_green, max_blue));
            println!("{}", sets_raw);
            println!("power {}", max_red * max_green * max_blue);

            return Game {
                id: game_id.parse().unwrap(),
                set: (max_red, max_green, max_blue),
                valid: game_valid,
            };
        })
        .collect();

    fn check_valid_set(set: &Vec<&str>) -> bool {
        let max_blue = 14;
        let max_red = 12;
        let max_green = 13;

        match *set.get(1).unwrap() {
            "blue" if set.get(0).unwrap().parse::<u32>().unwrap() > max_blue => false,
            "red" if set.get(0).unwrap().parse::<u32>().unwrap() > max_red => false,
            "green" if set.get(0).unwrap().parse::<u32>().unwrap() > max_green => false,
            _ => true,
        }
    }

    // PART 1
    println!(
        "{:?}",
        input.iter().filter(|x| x.valid).map(|x| x.id).sum::<u32>()
    );

    // PART 2
    println!(
        "{:?}",
        input.iter().map(|x| x.set.0 * x.set.1 * x.set.2).sum::<u32>()
    );
}
