#[allow(dead_code)]

enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug)]
enum Persons {
    Male(&'static str, u8),
    Female(&'static str, u8),
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

    let ppl: [Persons; 2] = [Persons::Male("timothy", 23), Persons::Female("tolu", 18)];

    println!("{:#?}", ppl);

    for p in ppl.iter() {
        print!(
            "{}, ",
            match p {
                Persons::Male(_, age) => age,
                Persons::Female(_, age) => age,
            }
        );
    }
}
