pub fn reply(message: &str) -> &str {
    match message {
        _x if message.trim().len() == 0 => "Fine. Be that way!",
        _x if is_question(message) && is_yelling(message) => "Calm down, I know what I'm doing!",
        _x if is_question(message) => "Sure.",
        _x if is_yelling(message) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}

pub fn is_question(s: &str) -> bool {
    s.trim().chars().last().unwrap() == '?'
}

pub fn is_yelling(s: &str) -> bool {
    s.trim()
        .chars()
        .all(|c| c.is_uppercase() || !c.is_alphabetic())
        && s.chars().any(|c| c.is_alphabetic() && c.is_uppercase())
}
