fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|line| line.split(":").last().unwrap().trim())
        .collect::<Vec<&str>>();

    println!("{:?}", input);
}
