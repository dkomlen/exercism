//If the number has 3 as a factor, output 'Pling'.
//If the number has 5 as a factor, output 'Plang'.
//If the number has 7 as a factor, output 'Plong'.
//If the number does not have 3, 5, or 7 as a factor, just pass the number's digits straight through.
pub fn raindrops(n: usize) -> String {
    let has_factor = |f: usize| -> bool { n % f == 0 };
    let mut ret = String::new();

    if has_factor(3) { ret += "Pling" }
    if has_factor(5) { ret += "Plang" }
    if has_factor(7) { ret += "Plong" }
    if ret.is_empty() { ret = n.to_string() }

    return ret;
}
