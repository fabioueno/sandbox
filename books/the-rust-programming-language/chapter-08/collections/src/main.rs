fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // The following creates a compilation error because it adds a mutable
    // reference (and we already have a immutable reference (first).
    // v.push(6);

    println!("The first element is: {first}");
}