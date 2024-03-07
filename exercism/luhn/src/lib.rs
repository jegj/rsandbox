/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let no_valid: bool = code.len() < 2;
    if no_valid {
        !no_valid
    } else {
        let reverse_code: String = code
            .to_string()
            .chars()
            .rev()
            .filter(|c| !c.is_whitespace())
            .collect();
        //println!("====>{} and orig=>{}", reverse_code, code);
        let mut acum = 0;
        for (index, digit) in reverse_code.chars().enumerate() {
            let number = digit.to_digit(10).unwrap();
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
        // println!("acum==>{}", acum);
        acum % 10 == 0
    }
}
