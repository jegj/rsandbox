/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    println!("====>{}", code);
    let no_valid: bool = code.len() <= 1;
    if no_valid {
        no_valid
    } else {
        let reverse_code: String = code
            .to_string()
            .chars()
            .rev()
            .filter(|c| !c.is_whitespace())
            .collect();
        println!("====>{}", reverse_code);
        return true;
    }
}
