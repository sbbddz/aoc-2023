fn part1() {
    let grid = include_str!("day3.txt")
        .lines()
        .map(|line| line.chars().map(|x| x.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    let mut valids = vec![];

    for x in 0..grid.len() {
        let mut future_num = String::from("");
        let mut has = false;
        for y in 0..grid.get(x).unwrap().len() {
            if let Ok(n) = grid.get(x).unwrap().get(y).unwrap().parse::<u32>() {
                if get_neighbours_part1(x, y, &grid) {
                    has = true;
                }

                future_num.push_str(&n.to_string());
            } else {
                if has == true {
                    println!("{}", future_num);
                    valids.push(future_num.parse::<u32>().unwrap());
                }
                has = false;
                future_num.clear();
            }
        }

        if has == true {
            valids.push(future_num.parse::<u32>().unwrap());
        }
    }

    println!("{:?}", valids.iter().sum::<u32>());
    fn get_neighbours_part1(x: usize, y: usize, grid: &Vec<Vec<String>>) -> bool {
        let mut n = vec![];

        if x != 0 {
            n.push(grid.get(x - 1).unwrap().get(y).unwrap());

            if y != 0 {
                // top-left
                n.push(grid.get(x - 1).unwrap().get(y - 1).unwrap());
            }

            if y != grid.get(0).unwrap().len() - 1 {
                // top-right
                n.push(grid.get(x - 1).unwrap().get(y + 1).unwrap());
            }
        }

        if x != grid.len() - 1 {
            n.push(grid.get(x + 1).unwrap().get(y).unwrap());

            if y != 0 {
                // bot-left
                n.push(grid.get(x + 1).unwrap().get(y - 1).unwrap());
            }

            if y != grid.get(0).unwrap().len() - 1 {
                // bot-right
                n.push(grid.get(x + 1).unwrap().get(y + 1).unwrap());
            }
        }

        if y != 0 {
            n.push(grid.get(x).unwrap().get(y - 1).unwrap());
        }

        if y != grid.get(0).unwrap().len() - 1 {
            n.push(grid.get(x).unwrap().get(y + 1).unwrap());
        }

        println!("{:?}, {:?}, {:?}", n, grid.get(x).unwrap().get(y).unwrap(), n.iter().any(|&x| x.chars().any(|x| x.is_ascii_punctuation() && x != '.')));
        return n.iter().any(|&x| x.chars().any(|x| x.is_ascii_punctuation() && x != '.'));
    }
}
