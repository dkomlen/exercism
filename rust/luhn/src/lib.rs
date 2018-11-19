/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let c = code.replace(" ", "");
    c.chars().enumerate().try_fold(0, |a, (i, c)| {
        match c.to_digit(10) {
            Some(n) => {
                let x = if i % 2 == 0 { n * 2 } else { n };
                if x > 10 { Ok(a + x - 9) } else { Ok(a + x) }
            }
            None => Err("Invalid code")
        }
    }).unwrap_or(1) % 10 == 0 && c.len() > 1
}