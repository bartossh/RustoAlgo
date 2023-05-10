pub fn brackets_are_balanced(string: &str) -> bool {
    let mut queue: Vec<char> = Vec::new();

    for c in string.chars() {
        match c {
            '{' => queue.push(c),
            '}' => {
                if !is_match(&'{', &mut queue) {
                    return false;
                }
            }
            '(' => queue.push(c),
            ')' => {
                if !is_match(&'(', &mut queue) {
                    return false;
                }
            }
            '[' => queue.push(c),
            ']' => {
                if !is_match(&'[', &mut queue) {
                    return false;
                }
            }
            _ => (),
        }
    }

    queue.len() == 0
}

fn is_match(c: &char, queue: &mut Vec<char>) -> bool {
    if let Some(v) = queue.pop() {
        return v == *c;
    }
    false
}
