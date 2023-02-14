#![allow(dead_code)]

#[derive(Debug, Clone)]
struct Pairs<T, R> {
    first: T,
    second: R,
}

impl <T: Copy, R: Copy> Pairs<T, R> {
    fn new(first: T, second: R) -> Self {
        Pairs {
            first, second
        }
    }

    fn from_tuple(val: (T, R)) -> Self {
        Pairs {
            first: val.0,
            second: val.1
        }
    }

    fn to_tuple(&self) -> (T, R) {
        (self.first(), self.second())
    }

    fn first(&self) -> T {
        self.first
    }

    fn second(&self) -> R {
        self.second
    }
}
