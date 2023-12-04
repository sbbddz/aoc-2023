#[derive(Debug)]
struct Gear<'a> {
    x: usize,
    y: usize,
    neighbors: Vec<(&'a String, usize, usize)>,
    gear_nums: Vec<u32>,
}

fn main() {
    let grid = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().map(|x| x.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    let mut gears = vec![];

    for x in 0..grid.len() {
        for y in 0..grid.get(x).unwrap().len() {
            let cell = grid.get(x).unwrap().get(y).unwrap();
            if cell == "*" {
                gears.push(Gear {
                    x,
                    y,
                    neighbors: get_neighbours(x, y, &grid),
                    gear_nums: vec![],
                });
            }
        }
    }

    let mut nums = vec![];

    for x in 0..grid.len() {
        let mut future_num = String::from("");
        let mut parsing_num = false;
        for y in 0..grid.get(x).unwrap().len() {
            let cell = grid.get(x).unwrap().get(y).unwrap();

            if let Ok(_) = cell.parse::<u32>() {
                parsing_num = true;
                future_num.push_str(cell);
            } else {
                if parsing_num {
                    nums.push((future_num.clone(), x, y));
                }
                parsing_num = false;
                future_num.clear();
            }
        }

        if parsing_num {
            nums.push((future_num.clone(), x, grid.get(x).unwrap().len()));
        }
    }

    for num in &nums {
        let num_range = num.2 - num.0.len()..num.2;
        let mut end_loop = false;
        for i in num_range {
            let found_gear = gears
                .iter_mut()
                .find(|x| x.neighbors.iter().any(|y| y.1 == num.1 && y.2 == i));
            if let Some(g) = found_gear {
                g.gear_nums.push(num.0.parse::<u32>().unwrap());
                end_loop = true;
                break;
            }
        }

        if end_loop {
            continue;
        }
    }

    let valid_gears = gears.iter().filter(|x| x.gear_nums.len() == 2);
    println!(
        "{:?}",
        valid_gears
            .map(|x| x.gear_nums.clone())
            .map(|x| x.iter().product::<u32>())
            .sum::<u32>()
    );
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

    n
}
