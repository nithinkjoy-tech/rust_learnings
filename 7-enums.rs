#[derive(Debug)]

enum Direction {
    East,
    West,
    North,
    South
}

fn main() {
    let direction=Direction::North;
    show_direction(direction);
}

fn show_direction(direction: Direction) {
    println!("Direction is {:?}",direction);
}
