#[allow(dead_code)]

enum Direction {
    North,
    South,
    West,
    East,
}

fn main() {
    let direction = Direction::South;

    println!(
        "{}",
        match direction {
            Direction::North => "NORTH",
            Direction::South => "SOUTH",
            Direction::East => "EAST",
            Direction::West => "WEST",
        }
    );
}
