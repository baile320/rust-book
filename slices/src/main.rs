fn main() {
    println!(
        "First word of 'Hello World' is {}",
        first_word(&String::from("Hello world!"))
    );
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s
}
