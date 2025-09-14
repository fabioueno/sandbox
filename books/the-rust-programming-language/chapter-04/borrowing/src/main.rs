fn main() {
    let s1 = String::from("Hello");
    let length = calculate_length(&s1);
    println!("The length of '{s1}' is {length}.\n");

    let mut s2 = String::from("Hello");
    borrow_data(&s2);                // Can't change the value.
    borrow_mutable_data(&mut s2);    // Can change the value.
    println!("{s2}? Just me...\n");  // s2 is still valid.

    let r1 = &mut s2;
    println!("{r1}");

    // This produces E0499 because there's already a mutable reference:
    // let r2 = &mut s5;

    // This produces E0502 because there's already a mutable reference:
    // let r3 = &s5;

    // The two errors only happen if this line is present. We need to actually
    // use the references to create a borrow error.
    // println!("r1: {r1}, r2: {r2}, r3: {r3}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn borrow_data(s: &String) {
    println!("{s} (borrowed).");

    // This produces E0596 because references are immutable:
    // s.push_str(", world!");
}

fn borrow_mutable_data(s: &mut String) {
    // This works because this reference is immutable:
    s.push_str(", world!");
}

// This produces E0106 because the function returns a reference to a local
// variable:
// fn dangle() -> &String {
//     let s = String::from("Hello");
//     &s
// }