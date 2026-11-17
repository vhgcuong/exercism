pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets: String = string.chars().filter(|c| "()[]{}".contains(*c)).collect();
    loop {
        match () {
            _ if brackets.contains("()") => brackets = brackets.split("()").collect(),
            _ if brackets.contains("[]") => brackets = brackets.split("[]").collect(),
            _ if brackets.contains("{}") => brackets = brackets.split("{}").collect(),
            _ => break,
        }
    }
    brackets.is_empty()
}
