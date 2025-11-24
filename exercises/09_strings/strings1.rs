// DONE: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    // String::from("blue") // explicit
    "blue".to_string() // idiomatic
    // "blue".into() // auto type-safe
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
