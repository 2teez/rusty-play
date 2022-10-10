trait HasSquareRoot {
    fn sq_root(self) -> Self;
}

impl HasSquareRoot for f32 {
    fn sq_root(self) -> f32 {
        self.sqrt()
    }
}

impl HasSquareRoot for f64 {
    fn sq_root(self) -> f64 {
        self.sqrt()
    }
}

fn quadratic_root<T>(value: T) -> T
where
    T: HasSquareRoot,
{
    value.sq_root().sq_root()
}

// a trait to implement a function length on &str
trait Counter {
    fn length(&self) -> usize;
}

impl Counter for &str {
    fn length(&self) -> usize {
        self.chars().count()
    }
}

fn main() {
    println!("{} {}", 33.142f32.sq_root(), 3.142f64.sq_root());
    println!(
        "{} {}",
        quadratic_root(33.142_f32),
        quadratic_root(3.142_f64)
    );

    println!("{}", quadratic_root("2.9680".parse::<f64>().unwrap()));
    println!("{}", "Uncle-ten..ten".length());
}
