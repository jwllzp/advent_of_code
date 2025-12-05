fn floor_n(x: i32, n: i32) -> i32 {
    if x >= 0 {
        x - x % n
    } else {
        (x % n - n) - x
    }
}

fn ceil_n(x: i32, n: i32) -> i32 {
    if x % n == 0 {
        x
    } else if x > 0 {
        x + (n - x % n)
    } else {
        x - x % n
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn floor_10(){
        assert_eq!(0, floor_n(3, 10));
        assert_eq!(0, floor_n(0, 10));
        assert_eq!(10, floor_n(10, 10));
        assert_eq!(-10, floor_n(-3, 10));
    }

    #[test]
    fn ceil_10(){
        assert_eq!(10, ceil_n(3, 10));
        assert_eq!(0, ceil_n(0, 10));
        assert_eq!(10, ceil_n(10, 10));
        assert_eq!(0, ceil_n(-3, 10));
    }
}
