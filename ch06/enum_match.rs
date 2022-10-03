#[allow(dead_code)]

enum Direction {
    North,
    South,
    West,
    East,
}

fn main() {
    let direction: Direction = Direction::South;

    println!(
        "{}",
        match direction {
            Direction::North => "NORTH",
            Direction::South => "SOUTH",
            Direction::East => "EAST",
            Direction::West => "WEST",
        }
    );

    // using guards in match
    for n in -2..=5 {
        println!(
            "{} {}",
            n,
            match n {
                0 => "zero",
                1 => "one",
                _ if n < 0 => "negative",
                _ => "plural",
            }
        );
    }
}
