#[allow(unused, dead_code)]

fn main() {
    let a: i8 = 23;
    let au: u8 = 23;
    let b: i16 = -280;

    let c: f64 = 10e78_f64;

    println!("{} {} {} {} {}", a, au, b, c, f64::INFINITY);

    // copied from Beginning Rust Book pg.80 just for reminder
    let _: i8 = 127;
    let _: i16 = 32_767;
    let _: i32 = 2_147_483_647;
    let _: i64 = 9_223_372_036_854_775_807;
    let _: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    let _: isize = 100;
    let _: u8 = 255;
    let _: u16 = 65_535;
    let _: u32 = 4_294_967_295;
    let _: u64 = 18_446_744_073_709_551_615;
    let _: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    let _: usize = 100;
    let _: f32 = 1e38;
    let _: f64 = 1e308;

    let mut counter = 0;
    for n in 32..256 {
        print!(
            "{} -> [{}], ",
            n,
            n as u8 as char,
            /*char::from_u32(n)*/
        );
        counter += 1;
        if 0 == counter % 8 {
            println!("\n");
        }
    }
}
