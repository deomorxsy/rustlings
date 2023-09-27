// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[derive(Debug)]
fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let start_index = input.to_string().chars().any(|j| !j.is_whitespace()).unwrap_or(0);
    let trimmed_start = &input[start_index..];

    let bytado = input.as_bytes();
    let end_index = bytado.iter().rposition(|index| !index.is_whitespace()).map(|position| position+1).unwrap_or(0);
    let trimmed_end = &input[..end_index];

    println!("bytado = {:#?}", bytado);

    return trimmed_start + trimed_end;

}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let refstr = input.to_string();
    let addstr = " world!";
    return format!("{}{}", refstr, addstr);
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    ???
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
