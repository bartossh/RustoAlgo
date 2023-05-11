pub fn reply(message: &str) -> &str {
    let msg = message.trim();

    if msg.len() == 0 {
        return "Fine. Be that way!";
    }

    if msg.to_ascii_uppercase() == msg && msg.to_lowercase().chars().any(|c| matches!(c, 'a'..='z'))
    {
        if msg.contains("?") {
            return "Calm down, I know what I'm doing!";
        }
        return "Whoa, chill out!";
    }
    if msg.chars().last() == Some('?') {
        return "Sure.";
    }

    "Whatever."
}
