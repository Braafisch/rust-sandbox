use std::char;
fn char_converter(input: char) -> char {
    if input.is_alphabetic() {
        let mut value = input.to_ascii_uppercase() as u8;
        value = (value + 13 - 65) % 26;
        (value + 65) as char
    } else {
        input
    }
}
fn ger_to_eng(input: &str) -> String {
    let mut result: String = "".into();
    for character in input.chars() {
        match character {
            'ä' => result += "ae",
            'ö' => result += "oe",
            'ü' => result += "ue",
            'Ä' => result += "AE",
            'Ö' => result += "OE",
            'Ü' => result += "UE",
            'ß' => result += "SS",
            _ => result.push(character),
        }
    }
    result
}
fn rot13(input: &str) -> String {
    let mut result: String = "".into();
    for character in ger_to_eng(input).chars() {
        result.push(char_converter(character));
    }
    result
}
fn main() {
    let result = rot13("Hallo, Welt!");
    println!("{0}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn When_Char_converter__Given_Char_H__Expect_Char_U() {
        let result = char_converter('H');
        assert_eq!(result, 'U');
    }
    #[test]
    fn When_Rot13__Given_String_Hallo__Expect_Char_UNYYB() {
        let result = rot13("Hallo");
        assert_eq!(result, "UNYYB");
    }
    #[test]
    fn When_GER_to_ENG__Given_GerString__Expect_Char_EngString() {
        let result = ger_to_eng("äüöÄÜÖß");
        assert_eq!(result, "aeueoeAEUEOESS");
    }
}
