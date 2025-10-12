// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    string_slice("blue");                                    // literal → &str

    string("red".to_string());                               // .to_string() → String

    string(String::from("hi"));                              // String::from() → String

    string("rust is fun!".to_owned());                       // .to_owned() → String

    string("nice weather".into());                           // .into() → String

    string(format!("Interpolation {}", "Station"));          // format! → String

    string_slice(&String::from("abc")[0..1]);                // slice [..] → &str

    string_slice("  hello there ".trim());                   // .trim() → &str

    string("Happy Monday!".replace("Mon", "Tues"));          // .replace() → String

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());         // .to_lowercase() → String
}
