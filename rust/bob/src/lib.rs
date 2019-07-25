pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let (question, yelling) = (is_question(message), is_yelling(message));

    if message.is_empty() {
        return "Fine. Be that way!";
    }
    match (question, yelling) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, _) => "Sure.",
        (_, true) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}

pub fn is_question(s: &str) -> bool {
    s.ends_with("?")
}

pub fn is_yelling(s: &str) -> bool {
    s.chars().all(|c| c.is_uppercase() || !c.is_alphabetic())
        && s.chars().any(|c| c.is_alphabetic() && c.is_uppercase())
}
