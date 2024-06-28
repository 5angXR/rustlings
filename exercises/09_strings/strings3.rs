// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    // let mut begin_i = 0;
    // let mut end_i = 0;
    
    // for i in 0..input.len() {
    //     if input.as_bytes()[i] == b' ' {
    //         continue;
    //     } else {
    //         begin_i = i;
    //         break
    //     }
    // }

    // for i in (0..input.len()).rev() {
    //     if input.as_bytes()[i] == b' ' {
    //         continue;
    //     } else {
    //         end_i = i;
    //         break
    //     }
    // }

    // let new_string = String::from(&input[begin_i..=end_i]);
    // new_string
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this!
    // let mut new_string = String::from(input);
    // new_string.push_str(" world!");
    // new_string
    
    format!("{} world!", input)
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    input.replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me(" Hello!     "), "Hello!");
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
