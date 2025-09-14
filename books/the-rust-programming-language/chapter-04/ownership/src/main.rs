fn main() {
    let s1 = String::from("Hello");

    // This produces E0382:
    // let s2 = s1;
    // println!("{s1}, world!");

    // This works but copies the data on the heap:
    let s2 = s1.clone();
    println!("{s1}, world!");
    println!("{s2} back to you!\n");

    // This works because fixed-size data live on the stack:
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}\n");

    let s3 = String::from("What");
    take_ownership(s3);

    // This produces E0382 too, because the ownership
    // of s3 has been moved to the function:
    // println!("{s3} back to you!");

    let i = 5;
    copy_data(i);

    println!("i is valid: {i}\n");

    let s4 = give_ownership();
    println!("'{s4}' is also valid!");
}

fn take_ownership(s: String) {
    println!("{s} moved to function.\n");
}

fn give_ownership() -> String {
    let s = String::from("This");
    s
}

fn copy_data(i: u32) {
    println!("{i} copied to function.");
}