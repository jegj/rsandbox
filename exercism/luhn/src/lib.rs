/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    const INVALID: u32 = 1_000_000_000;
    let reverse_code: String = code
        .to_string()
        .chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .collect();

    let no_valid: bool = reverse_code.len() < 2;
    if no_valid {
        !no_valid
    } else {
        //println!("====>{} and orig=>{}", reverse_code, code);
        let mut acum = 0;
        let mut valid = true;
        for (index, digit) in reverse_code.chars().enumerate() {
            let number = digit.to_digit(10).unwrap_or(INVALID);
            if number == INVALID {
                valid = false;
                break;
            }
            if index % 2 != 0 {
                let double_digit = number * 2;
                //println!("->{}", double_digit);
                if double_digit >= 10 {
                    //println!("->{}", double_digit - 9);
                    acum += double_digit - 9;
                } else {
                    //println!("->{}", number);
                    acum += double_digit;
                }
                //println!("===>{} at index {}", digit, index);
            } else {
                //println!("->{}", number);
                acum += number;
            }
        }
        //println!("acum==>{}", acum);
        if valid {
            acum % 10 == 0
        } else {
            valid
        }
    }
}
