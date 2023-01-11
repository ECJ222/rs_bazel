pub fn find_substring<'a>(s: &'a str, substring: &str) -> Option<&'a str> {
    s.find(substring).map(|i| &s[i..i + substring.len()])
}

pub fn replace_substring(s: &str, from: &str, to: &str) -> String {
    s.replace(from, to)
}

#[cfg(test)]
mod tests {
    use crate::{find_substring, replace_substring};

    #[test]
    fn find() {
        let s = "Dragons fly!";
        let substring = find_substring(s, "fly");
        assert_eq!(substring, Some("fly"));
    }

    #[test]
    fn replace() {
        let s = "Hello, World!";
        let new_string = replace_substring(s, "World", "Rust");
        assert_eq!(new_string, "Hello, Rust!");
    }
}
