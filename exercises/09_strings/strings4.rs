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
    string_slice("blue"); // &str

    string("red".to_string()); // String

    string(String::from("hi")); // String

    string("rust is fun!".to_owned()); // String

    string_slice("nice weather".into()); // &str

    string(format!("Interpolation {}", "Station")); // String

    string_slice(&String::from("abc")[0..1]); // &str

    string_slice("  hello there ".trim()); // &str

    string("Happy Monday!".replace("Mon", "Tues")); // String

    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // String
}

