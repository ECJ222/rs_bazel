extern crate substring_library;

use substring_library::{find_substring, replace_substring};

fn main() {
    let s = "Hello, World!";

    let substring = find_substring(s, "World");
    println!("Found substring: {:?}", substring);

    let new_string = replace_substring(s, "World", "Rust");
    println!("New string: {}", new_string);
}
