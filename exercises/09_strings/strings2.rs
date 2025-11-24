// DONE: Fix the compiler error in the `main` function without changing this function.
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green"); // Don't change this line.

    // // if is_a_color_word(word.into()) { // ! the trait `From<String>` is not implemented for `&str`
    if is_a_color_word(&word) { // most idiomatic
    // if is_a_color_word(word.as_str()) { // * equiv to above because `String` impls `Deref<Target = str>`
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}
