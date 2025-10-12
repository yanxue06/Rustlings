fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    let new_input = input.trim_start();
    new_input.trim_end()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    let mut new_str = input.to_string(); 
    new_str.push_str(" world!"); 
    new_str
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    let new_input = input.replacen("cars", "balloons", 1); 
    new_input.to_string() 
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
        assert_eq!(trim_me("Hi!"), "Hi!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
