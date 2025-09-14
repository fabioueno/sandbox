fn main() {
    let s = String::from("Hello world");

    let h1 = &s[0..5];
    let w1 = &s[6..11];

    let h2 = &s[0..=4];
    let h3 = &s[..5];

    let w2 = &s[6..=10];
    let w3 = &s[6..];

    println!("{s}");
    println!("{h1} {w1}");
    println!("{h2} {w2}");
    println!("{h3} {w3}\n");

    let first = first_word(&s);
    println!("First word is '{first}'");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[0..]
}