// We need to annotate lifetimes on references in structs.
struct Excerpt<'a> {
    x: &'a str
}

// We need to annotate the lifetime in (some) functions.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    println!("{}", longest("abc", "defgh"));
}