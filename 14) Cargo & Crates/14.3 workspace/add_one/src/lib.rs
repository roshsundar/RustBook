use rand::{self, Rng};

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn rand_num(lower: i32, upper: i32) -> i32 {
    let mut rng = rand::rng();
    rng.random_range(lower..upper)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}