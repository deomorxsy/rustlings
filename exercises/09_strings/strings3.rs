// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

//#[derive(Debug)]
fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let firstValue = input.to_string();
    let mut fv_char = firstValue.chars();
    // if the left end starts with whitespace, find the end of the whitespace
    let fv_pos = fv_char.position(|j| !j.is_whitespace());
    let start_index =  fv_pos.unwrap_or(0);


    // rposition requires a reversible iterator. It also needs a exact-size iterator,
    // so it is possible to assign indices the same way as position.
    // A exact-size iterator implements the std::iter::ExactSizeIterator trait,
    // but since not every iterator knows ahead of time how many items it will produce
    // (like .chars() which is used in the fv_char variable, which does not know AOT),
    // it cant be used on strings. Hence a byte array iterator is used instead, which
    // implements ExactSizeIterator.
    // And hence, is_ascii_whitespace is needed.
    let fv_bytado =  input.as_bytes();
    let fv_rev_pos = fv_bytado.iter().rposition(|j| !j.is_ascii_whitespace());

    // last index
    let end_index = fv_rev_pos.unwrap_or(0);
    // slice the string with the two indexes and remove whitespace. +1 ftw
    let final_trimado = &input[start_index..end_index+1];

    return final_trimado.to_string();
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let refstr = input.to_string();
    let addstr = " world!";
    return format!("{}{}", refstr, addstr);
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let inp_char = input.to_string();
    //repl_char.replace("cars", "balloons");
    if inp_char.contains("cars") {
        let replaced = inp_char.replace("cars", "balloons");
        return replaced;
    } else {
        return inp_char;
    }
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
