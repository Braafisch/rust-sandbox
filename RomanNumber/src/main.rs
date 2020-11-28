fn convert_to_i32(letter: char) -> Result<i32, char> {
    match letter {
        'I' => Ok(1),
        'V' => Ok(5),
        'X' => Ok(10),
        'L' => Ok(50),
        'C' => Ok(100),
        'D' => Ok(500),
        'M' => Ok(1000),
        _ => Err(letter),
    }
}

fn roman_num(letters: &str) -> i32 {
    let mut number = 0;
    let mut curr = 0;
    let mut prev = 0;
    let mut prevprev = 0;
    let mut count = 0;

    for letter in letters.chars().rev() {
        curr = convert_to_i32(letter).expect("Invalid roman number!");
        number += if curr < prev { -curr } else { curr };
        count += if curr == prev { 1 } else { -count };

        if count >= 3
            || count == 1 && (prev < prevprev || (curr as f64).log10() % 1.0 != 0.0)
            || curr < prev / 10
            || curr == prev / 2
            || curr < prev && prev == prevprev
        {
            panic!("Invalid roman number! {0}", letters)
        }
        prevprev = prev;
        prev = curr;
    }
    number
}

fn main() {
    let input = "IV";
    let num = roman_num(input);
    println!("{0} -> {1}", input, num);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest(
        letter,
        expected,
        case('I', 1),
        case('V', 5),
        case('X', 10),
        case('L', 50),
        case('C', 100),
        case('D', 500),
        case('M', 1000)
    )]
    fn When_roman_number__Given_One_Diget_Roman_GoodNumber__Expect_Number(
        letter: char,
        expected: i32,
    ) -> Result<(), String> {
        assert_eq!(convert_to_i32(letter)?, expected);
        Ok(())
    }
    #[rstest(
        letter,
        case('B'),
        case('T'),
        case('F'),
        case('G'),
        case('H'),
        case('K'),
        case('R')
    )]
    fn When_roman_number__Given_One_Diget_Roman_BadNumber__Expect_Error(
        letter: char,
    ) -> Result<(), String> {
        assert_eq!(convert_to_i32(letter), Err(letter));
        Ok(())
    }
    #[rstest(
        letters,
        expected,
        case("IV", 4),
        case("VI", 6),
        case("IX", 9),
        case("XI", 11),
        case("XX", 20),
        case("XL", 40),
        case("LX", 60),
        case("XC", 90),
        case("CX", 110),
        case("CC", 200),
        case("CD", 400),
        case("DC", 600),
        case("CM", 900),
        case("MC", 1100),
        case("MM", 2000)
    )]
    fn When_roman_number__Given_Two_Diget_Roman_GoodNumber__Expect_Number(
        letters: &str,
        expected: i32,
    ) {
        let result = roman_num(letters);
        assert_eq!(result, expected);
    }
    #[rstest(letters, case("AB"), case("KX"), case("VV"), case("LL"), case("DD"))]
    #[should_panic]
    fn When_roman_number__Given_Two_Diget_Roman_BadNumber__Expect_Panic(letters: &str) {
        let result = roman_num(letters);
    }
    #[rstest(
        letters,
        expected,
        case("VII", 7),
        case("XIII", 13),
        case("MCMXCIX", 1999),
        case("CXX", 120),
        case("LXXX", 80),
        case("CMX", 910),
        case("LXIV", 64)
    )]
    fn When_roman_number__Given_Big_Roman_GoodNumber__Expect_Number(letters: &str, expected: i32) {
        let result = roman_num(letters);
        assert_eq!(result, expected);
    }
    #[rstest(
        letters,
        case("CMMD"),
        case("IIV"),
        case("VIIII"),
        case("IIII"),
        case("IM")
    )]
    #[should_panic]
    fn When_roman_number__Given_Big_Roman_BadNumber__Expect_Panic(letters: &str) {
        let result = roman_num(letters);
    }
}
