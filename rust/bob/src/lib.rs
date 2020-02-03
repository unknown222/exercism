pub fn reply(message: &str) -> &str {

    let has_letters = message.chars().any(char::is_alphabetic);
    let trimmed_message = message.trim();

    if trimmed_message.is_empty() {
        return "Fine. Be that way!";
    }

    if has_letters && &trimmed_message.to_uppercase() == trimmed_message && trimmed_message.ends_with("?") {
        return "Calm down, I know what I'm doing!";
    }

    if has_letters && &trimmed_message.to_uppercase() == trimmed_message {
        return "Whoa, chill out!";
    }

    if trimmed_message.ends_with("?") {
        return "Sure.";
    }
    if trimmed_message.ends_with("!") {
        return "Whatever.";
    }

    return "Whatever.";
}
