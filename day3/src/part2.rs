fn main() {
    let grid = include_str!("day3.txt")
        .lines()
        .map(|line| line.chars().map(|x| x.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    for x in 0..grid.len() {
        for y in 0..grid.get(x).unwrap().len() {
            let cell = grid.get(x).unwrap().get(y).unwrap();
            if cell == "*" {
                let nbours = get_neighbours(x, y, &grid);
                let mut numeric_nbours: Vec<_> = nbours
                    .iter()
                    .filter(|&x| x.0.chars().any(|y| y.is_digit(10)))
                    .collect();

                for nnb in &numeric_nbours {
                    let num = get_nearer_num(nnb.1, nnb.2, &grid);
                    println!("{}", num);
                }
            }
        }
    }
}

fn get_nearer_num(x: usize, y: usize, grid: &Vec<Vec<String>>) -> &String {
    return grid.get(x).unwrap().get(y).unwrap();
}

fn get_neighbours(x: usize, y: usize, grid: &Vec<Vec<String>>) -> Vec<(&String, usize, usize)> {
    let mut n = vec![];

    if x != 0 {
        n.push((grid.get(x - 1).unwrap().get(y).unwrap(), x - 1, y));

        if y != 0 {
            // top-left
            n.push((grid.get(x - 1).unwrap().get(y - 1).unwrap(), x - 1, y - 1));
        }

        if y != grid.get(0).unwrap().len() - 1 {
            // top-right
            n.push((grid.get(x - 1).unwrap().get(y + 1).unwrap(), x - 1, y + 1));
        }
    }

    if x != grid.len() - 1 {
        n.push((grid.get(x + 1).unwrap().get(y).unwrap(), x + 1, y));

        if y != 0 {
            // bot-left
            n.push((grid.get(x + 1).unwrap().get(y - 1).unwrap(), x + 1, y - 1));
        }

        if y != grid.get(0).unwrap().len() - 1 {
            // bot-right
            n.push((grid.get(x + 1).unwrap().get(y + 1).unwrap(), x + 1, y + 1));
        }
    }

    if y != 0 {
        n.push((grid.get(x).unwrap().get(y - 1).unwrap(), x, y - 1));
    }

    if y != grid.get(0).unwrap().len() - 1 {
        n.push((grid.get(x).unwrap().get(y + 1).unwrap(), x, y + 1));
    }

    return n;
}
