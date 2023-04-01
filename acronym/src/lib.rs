pub fn abbreviate(phrase: &str) -> String {
    let cs = phrase.chars();
    let mut result: String = String::new();

    let mut make_upper = true;
    let mut upper_before = false;

    for c in cs.into_iter() {
        match true {
            _ if { !c.is_alphabetic() && c != '\'' } => {
                make_upper = true;
                upper_before = false;
            }
            _ if { make_upper || (c.is_uppercase() && !upper_before) } => {
                result.push(c.to_ascii_uppercase());
                make_upper = false;
                upper_before = true;
            }
            _ if { c.is_uppercase() } => {
                upper_before = true;
            }

            _ => {
                make_upper = false;
                upper_before = false;
            }
        }
    }

    result
}
