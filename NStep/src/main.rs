fn NStep(x: i32, y: i32) -> Result<i32, &'static str> {
    if x == y || (x - 2) == y {
        Ok(x + y - if x % 2 == 0 { 0 } else { 1 })
    } else {
        Err("No Number")
    }
}

fn main() {
    NStep(0, 1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn When_NStep__Given_XZeroYZero__Expect_Zero() -> Result<(), String> {
        assert_eq!(NStep(0, 0)?, 0);
        Ok(())
    }

    #[rstest(
        x,
        y,
        expected,
        case(1, 1, 1),
        case(2, 0, 2),
        case(3, 1, 3),
        case(2, 2, 4),
        case(4, 2, 6),
        case(3, 3, 5),
        case(5, 3, 7),
        case(4, 4, 8),
        case(6, 4, 10),
        case(5, 5, 9),
        case(7, 5, 11),
        case(6, 6, 12)
    )]
    fn When_NStep__Given_GoodXY_Expect_Number(x: i32, y: i32, expected: i32) -> Result<(), String> {
        assert_eq!(NStep(x, y)?, expected);
        Ok(())
    }

    #[rstest(
        x,
        y,
        case(0, 1),
        case(0, 2),
        case(0, 3),
        case(0, 4),
        case(0, 5),
        case(1, 0),
        case(3, 0),
        case(4, 0),
        case(5, 0),
        case(6, 0)
    )]
    fn When_NStep__Given_BadXY_Expect_NoNumber(x: i32, y: i32) -> Result<(), String> {
        assert_eq!(NStep(x, y), Err("No Number"));
        Ok(())
    }
}
