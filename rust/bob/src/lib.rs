pub fn reply(message: &str) -> &str {
    match message.trim() {
        s if s.is_empty() => "Fine. Be that way!",
        s if is_all_capital(s) => {
            if s.ends_with("?") {
                "Calm down, I know what I'm doing!"
            } else {
                "Whoa, chill out!"
            }
        },
        s if s.ends_with("?") => "Sure.",
        _ => "Whatever."
    }
}

fn is_all_capital(message: &str) -> bool {
    let mut capital = false;
    for c in message.chars() {
        if c.is_alphabetic() {
            if c.is_lowercase() {
                return false;
            } else {
                capital = true;
            }
        }
    }
    return capital;
}